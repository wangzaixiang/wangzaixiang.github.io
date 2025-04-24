# SQL JOIN

# 参考
## Swiss Table / Hashbrown 
1. Video [Designing a Fast, Efficient, Cache-friendly Hash Table, Step by Step](https://www.youtube.com/watch?v=ncHmEUmJZf4)
2. Slides: [Designing a Fast, Efficient, Cache-friendly Hash Table, Step by Step](https://hackmd.io/@heyfey/SJZ-3jbs5)

Swiss Table 是 Rust HashMap 的缺省实现，其内部使用 SIMD 友好的算法，这对于 ht.lookup(key) 来说，是做了充分的向量化优化的，因此，相比传统的
数组 + 链表方式会有很大的性能提升，不过，Swiss Table 是在单次 lookup(key) 内进行向量加速的，另外一个视角则是 lookup(key_vec), 而且由于 数据库的 hash-join
是一个特定问题，有以下因素可能都会影响到 join 的性能：
- build table 不涉及到 delete/update 的操作，无需考虑这一块的支持。
- build table 的 hash-column 如果是已排序的，则 lookup 也可以利用这一特性进行加速（例如提前终止搜索）

## Balancing vectorized query execution with bandwidth-optimized storage
文章解读：[](https://www.cockroachlabs.com/blog/vectorized-hash-joiner/)
1. 5.3 Case Study: HashJoin 
   本节提出了一个巧妙的向量化的 HashJoin 算法，其核心是使用了一个巧妙的 next 链表。
   TODO 使用几个图来说明这个算法的原理。

   - 我有一个改进的想法：当 build table 很大时，引入两个新的存储，来进一步改善内部局部性
     
     文中的算法，next 链表是跳跃的，对同一个 hash 值的数据，其需要多次内存地址跳转才能满足。
     - hash index: `[ hashcode: (offset-a, count) ]`
     - a: `[offset: (row-no, build-value)`] 从offset开始的连续 N 行都是具有相同的hashcode，直到 row-no = -1 结束
     将 next 链表从跳跃方式改为连续模式。
     这会增加 build 的开销，但在 probe 时，可以带来更佳的内存局部性，这样 build 侧进行分区的必要性就可能大为降低。
2. 5.4.1 Best Effort Partitioning 

   分区对 hash join 的优化：
   - build table 更小，从而带来更好的内存局部性
   - 如果 probe table 与 build table 有相同的分区设置，则在 probe 分区完成后，build table 占用的内容可以及时释放
   - 结合分区、分桶策略，可以实现更细粒度的 partition hash join.
   - 如果 probe table 与 build table 以及 hash-table 的分区策略不同，则会导致 shuffle 开销。
     - 一般来说，probe 侧的数据量更大，probe 应避免 shuffle。

## [A Vectorized Hash-Join](https://groups.csail.mit.edu/cag/6.893-f2000/vectorhashjoin.pdf)
1. 2.3 Grace Hash-Join
   - 将 hash-table 分成多个 partition，避免 hash-table 过大。每个 partition 可以在 memory 中，或置换到 disk 中。
   - probe-table 也需要分成多个 partition，
   - 然后逐个 partition 进行 hash-join

## ClickHouse Join Stratage
1. [Join Types supported in ClickHouse](https://clickhouse.com/blog/clickhouse-fully-supports-joins-part1)
2. [Hash Join, Parallel HashJoin, Grace HashJoin](https://clickhouse.com/blog/clickhouse-fully-supports-joins-hash-joins-part2)
3. [Full Sort Merge, Partial Merge Jooin](https://clickhouse.com/blog/clickhouse-fully-supports-joins-full-sort-partial-merge-part3)
4. [Direct Join](https://clickhouse.com/blog/clickhouse-fully-supports-joins-direct-join-part4)
