# DataFusion HashJoin 源代码阅读

# Concepts
1. left side: build side, 完整读取，构建 hash table
2. right side: probe side: 分批读取，并进行 hash-join

# flow

## process_probe_batch (40%+18%+8.5%+8.0% = 74.5%)
对 case45 这个 join 查询，process_probe_batch 的耗时总占比高达 74.5%，是这个查询性能优化的主要点。

1. 当前环境
2. let (left_indices: UInt64Array, right_indices: UInt64Array, next_offset: Option<JoinHashMapOffset> ) = lookup_join_hashmap(...);
   根据 hash 值，计算一个 (build_indices, probe_indices) 的列表，表示这两个 indices 之间的 关联列 相等。目前，这一步的耗时是最多的(74%, 30%+11%+7.7%+7.1%=55.8%)。
   1. build_hashmap.get_matched_indices_with_limit_offset
      - hash_values: &[u64]
   2. equal_row_arr
3. 处理 filter, 类似于 `from a join b on a.id = b.id and b.age > 10` 这样的过滤，裁剪掉 (build_indices, probe_indices) 元组
4. adjust_indices_by_join_type: 对 left/right join, 补充上缺失的 pair。(耗时约 14%, 4.1%+5.2%+0.6%+0.6%=10.5%)
5. build_batch_from_indices: 根据 left_indices, right_indices, 生成新的 batch，返回给上层的 pipeline （耗时约 9.5%, 4.6%+2.1%+0.1%+0.2% = 7% ）。

## samply 使用技巧
- 合并函数: 将函数从调用栈上删除（减少调用栈层次）
- 只合并节点
- 聚焦于函数： 以当前函数为栈顶（多个caller信息的信息会合并，多个子树）
- 只聚焦于子树：只聚焦于当前子树。
- 聚焦于分类 Regular
- 折叠函数。 不显示这个函数的 callee 信息 
- 折叠 project：不显示当前项目模块中的函数开销
- 丢弃与此函数相关的样本：从当前火焰图中删除所有这个函数（不仅当前子树）

##
1. JoinHashMap
   - map: HashTable<(u64, u64)>
## 其他开销
忽略 process_probe_batch 后，其他几块的开销主要是：
1. datafusion_physical_plan::filter::filter_and_project 13%
2. datafusion_physical_plan::coalesce::BatchCoalescer::push_batch 46%
3. datafusion_physical_plan::joins::hash_join::update_hash 11%

1. 按照目前的设计，JoinHashMap 中 next 是非常稀疏的。 如果 build table 使用 join_key 作为主键时，一般没有 next。
   - 优化：在 hash_map 中存储 index 使用最高位作为 next 标记：为1时表示有 next，为 0 时表示无。在大部份情况下，仅需一次 lookup 而无需 next 处理。
   - vec.push 操作优化，避免内存分配。
   - 优化2: 重新设计 next 结构，使得更为 cache local，避免反复的跳转。

疑点：
1. get_matched_indices_with_limit_offset > chain_traverse 并没有真实的并行化
   - vec.push 有额外的开销，包括容量检查等，内存分配等
   - 循环过程没有很好的向量化。
   - 尝试写一个单元测试，看看 IPC 如何？
   - next 数组是否缓存友好？


# todos
-[ ] how to modify and debug 3rd party crate?
    1. update source at ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crate-name-version/src/some.rs
    2. rm -fr target/debug/.fingerprint/crate-name-hash/
    3. cargo build 会重新编译该模块。 
    4. cargo clean 不会删除 ~/.cargo/registry 下的文件

    or
    1. git clone 
    2. Cargo.toml, change as `datafusion = { path = "/Users/wangzaixiang/workspaces/github.com/datafusion/datafusion/core", version = "46.0.1"}`
    3. 修改了文件时，cargo build 会重新编译该模块，方便进行代码的调试。

- ProcessProbeBatchState
    - offset?
    - joined_probe_idx
- partition
    - 如何仅在某个 partition 进行调试？
    - 如何设定近使用1个 partition?
    - 阅读代码，理解如何从 LogicPlan 生成 PhysicalPlan
    config.optimizer.repartition_joins
    ```rust
     let config = SessionConfig::new().with_repartition_joins(false);
     let ctx = SessionContext::new_with_config(config);
    ```
- [Coroutine 重构数据库算子——以 hash join probe 为例](https://zhuanlan.zhihu.com/p/666465496?utm_medium=social&utm_psn=1897664392729982364&utm_source=ZHShareTargetIDMore)
  对使用 coroutine 来提高这一块的性能表示怀疑，主要是因为 coroutine 的调度开销会更大。
  
- 新的向量算法
  ```rust
      struct FirstEntry {
        // hash: u64,
        first: u64,  // 0 表示没有 first
        next:  u64,  // 0 表示没有 next
        value: u64,
      }
      struct NextEntry {
        // hash: u64,
        index: u64, // <= 0x7FFF_FFFF_FFFF_FFFF means no next, otherwise next
        value: u64
      }
  
      struct HashMap {
          firsts: Vec<FirstEntry>,
          nexts: Vec<NextEntry>,
      }
        
      fn lookup(hm:&HashMap, probe: Vec<u64>) -> (Vec<u64>, Vec<u64>) {
          // 每次循环处理 4 个 probe
          let probe1: u64x4 = u64x4::from(&probe[0]);
          let index1: u64x4 = probe1 % hm.firsts.len() as u64;    // 1 次随机访问
          let build_index: u64x4 = hm.firsts[index1].first;
          let build_next: u64x4 = hm.firsts[index1].next;
          let build_value: u64x4 = hm.firsts[index1].value;
            
          let m1: u64x4 = first > 0;
          let eq: u64x4 = m1 & (first == probe1);
            
          // 每次最多输出 4 个结果
          // emit eq records
            
          let hash_next = build_next != 0;
          while unlikely(has_next) {
              let build_index: u64x4 = hm.nexts[build_next].index;
              let build_value: u64x4 = hm.nexts[build_next].value;
              let build_next: u64x4 = build_next + hash_next;
                
              let m1: u64x4 = build_index > 0;
              let eq: u64x4 = m1 & (build_value == probe1);
    
              // emit eq records
          }
    
      }
  
  ```
  
- 性能优化
  1. 基于 profile 的性能优化：将执行过程中的时间信息收集起来，反馈到 LogicPan 中，下次执行时，根据 profile 信息优化查询计划
     - 根据左表、右表的数据规模选择 build side
     - 消除不必要的 re-partition （数据量小于 1M 时，直接不做 partion 处理）
 