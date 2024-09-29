# Execution

学习 duckdb 的源代码，我是首先从 Execution 这一部份开始的：
- 虽然从结构上来看，Execution 是整个系统的最后一步，但秉着“以终为始”的学习原则， 从 Execution 开始，可以更好的了解整个系统的核心。从
  Parser 到 Logical Plan 到 Optimizer 到 Physical Plan，都是在为 Execution 这一步做准备。
- DuckDB 的性能优化的关键，也是在 Execution 这一步，其核心是 Pipeline 的机制和 Vector 的数据结构。

理解了 Pipeline 和 Vector，再结合几个主要的 Operator 原理，就可以对 DuckDB 的执行机制有一个初步的认识了。

## 参考文档
1. DuckDB internals
   - [Slides](https://15721.courses.cs.cmu.edu/spring2023/slides/22-duckdb.pdf),
   - [Video](https://www.youtube.com/watch?v=bZOvAKGkzpQ)
2. Push-Based Execution In DuckDB
   - [Slides](https://dsdsd.da.cwi.nl/slides/dsdsd-duckdb-push-based-execution.pdf),
   - [Video](https://www.youtube.com/watch?v=1kDrPgRUuEI)
3. 知乎链接 [DuckDB Push-Based Execution Model](https://zhuanlan.zhihu.com/p/402355976) 


接下来，我们将从一些简单的SQL 执行示例出发，来阅读 Execution 模块的源代码。
1. [example query: 两个表join后分组聚合](execution-demo1.md)