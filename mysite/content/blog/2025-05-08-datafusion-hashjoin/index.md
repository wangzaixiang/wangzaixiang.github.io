+++
title = "datafusion hashjoin 性能分析"
description = "datafusion 中的 hashjoin 算子进行性能分析，并尝试进行优化。"
date = 2025-05-08
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

最近，阅读了 datafusion 项目的源代码，我会通过这篇文章，整理代码阅读中的一些笔记、思考。本文目前是处于草稿阶段（凌乱），会持续修改、更新。

# datafusion 性能测评

# Top-down pull execution(Volcano) vs bottom-up push execution(Pipeline)

# HashJoin 性能测评
1. in-order vs out-of-order
2. small build set vs large build set

# HashJoin 改进
1. Query Plan 改进
   - left join 改写为 inner join
   - filter move to left side
2. HashJoin 算子改进
   - 使用行存替代列存，以提高缓存有好性
   - 显示的 SIMD 优化
   - 新的 hashmap 数据结构设计

## 改进的 HashMap 数据结构
```rust

struct HashMap1 { 
   tags: Box<[u16]>,    // 每 16 个 tag 为 1 group,  group_idx = h1(hash) % GROUP, tag = h2(hash)
   first: Box<[u64]>,   // first.len() == tags.len(), 存储每个 tag 对应的 fist 行号
   next:  Box<[u64]>    // next.len() == ROWS + 1, 存储每个行号对应的下一行号， 0 表示无。
}

struct HashMap2 {
   tags: Box<[u16]>, 
   first: Box<[u16]>,   // first.len() == tags.len(), first[i] 为 rows 的索引，0 表示无数据
   rows: Box<[Row]>,    // 采用 row storage，其顺序不同于输入的顺序，而是按照在 hashmap 后的顺序
}

struct Row {
   hashcode: u64,   // 仅保留 63位，最高位为0时，表示没有下一行，为1时，表示有下一行。
   col1:  any,      // 保留原始的数据列
   col2:  any,
}
```

1. build hashmap1
   - GROUPS = ROWS * SCALE / 16, SCALE 建议选择 2-4，每个 groups 中的 tag 都初始化为 0xFFFF(每个分组非最后一个slot) 和 0xFFFE(每个分组最后一个slot)
   - foreach build row, calculate hashcode: u64 for hash-columns
   - h2 = hashcode >> 49: 保留15位的 h2
   - group_idx = hashcode % GROUPS
   - 在 1 个 groups 中使用 simd 指令搜索 h2
     - (unlikely) 如果存在，则表示该 h2 已被占用，`next[row_no] = first[slot], first[slot] = row_no`
     - (likely) 如果不存在，则搜索 0xFFFF 找到第一个可用 slot
       - (likely) 如果找到，则占用该 slot：`first[slot] = row_no`
       - (unlikely) 如果没有找到，则占用最后一个slot：`next[row_no] = first[slot], first[slot] = row_no`
2. build hashmap2
   - 遍历 hashmap1.tags
   - 如果某个 tag 对应的 first != 0，则将 first 行压入 rows
     - 遍历 next 栈，将当前 tag 的下一个 next 压入rows， 直至结束
   hashmap2 构建完成后，之前的列存数据可以释放。hashmap2 中使用行存进行存储，且具有相同的 hashcode 的行被设计为连续存储。为后续 probe phase 的性能优化提供基础。

### 实验数据
在 https://github.com/wangzaixiang/vectorize_engine/blob/main/playgrounds/try_cpu/src/bin/TestHash.rs 这个实验中，我对这个数据
结构进行了测试

| GROUP_WIDTH | H2_WIDTH | SCALE | H2 duplicated | other duplicated |
|-------------|----------|-------|---------------|------------------|
| 16          | 8        | 1     | 11.37%        | 9.32%            |
| 16          | 8        | 2     | 6%            | 0.098%           |
| 16          | 8        | 3     | 4%            | 0.002%           |
| 16          | 8        | 4     | 3.09%         | 0.001%           |
| 16          | 16       | 1     | 0.05%         | 13.24%           |
| 16          | 16       | 2     | 0.03%         | 0.18%            |
| 16          | 16       | 3     | 0.02%         | 0.004%           |
| 16          | 16       | 4     | 0.01%         | 0%               |

实验表明：选择 (GROUP_WIDTH = 16, H2_WIDTH = 16, SCALE = 2) 时，first 命中率达到 99.9%，效果很理想，远比 (GROUP_WIDTH = 16, H2_WIDTH = 8, SCALE = 2)
要好。而其成本增加较为有限：
1. tags 占用空间增加1倍
2. 搜索时，需要搜索 32B(vs 16B)


# probe phase


# Mac 下的性能测评工具

# datafusion 文章系列
1. [push vs pull](@/blog/2025-04-08-duck-push-vs-datafusion-pull/index.md)
2. [datafusion hashjoin executor](@/blog/2025-05-08-datafusion-hashjoin/index.md)
3. [datafusion window function executor](@/blog/2025-05-26-datafusion-window-function/index.md)
4. [datafusion performance](@/blog/2025-06-06-datafusion-performance/index.md)

