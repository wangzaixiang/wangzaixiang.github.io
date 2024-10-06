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

## 代码调试技巧
1. 在 duckdb shell 中 使用 `explain statement` 或者`explain analyze statement` 查看执行计划。
2. PipelineExecutor::Execute(idx_t max_chunks) 是 Pipeline 的执行入口，可以从这里添加断点，开启一个 Pipeline 的调试。
   -. watches: `this->pipeline.ToString()` 查看当前 Pipeline 的信息。 或者 `this->pipeline.Print()` 在 console 中
      查看当前 Pipeline 的 信息。 一次SQL 执行会产生多个 Pipeline，仅在满足条件的 Pipeline 上设置断点。
   -. watches: `this->pipeline.source->ToString(ExplainFormat::TEXT)` 查看当前 Pipeline 的 Source 信息。
   -. 在调试的 Variables 面板中，可以查看 pipeline 的 source, operators, sink 的信息，在对应的 operator 上设置断点。
3. source 节点的入口是: `PhysicalXxxOperator::GetData(ExecutionContext &context, DataChunk &chunk, OperatorSourceInput &input)`， 
   可以在这里设置断点，查看 source 的执行流程。
4. operator 节点的执行入口是：`PhysicalXxxOperator::Execute(ExecutionContext &context, DataChunk &input, DataChunk &chunk,
   GlobalOperatorState &gstate, OperatorState &state`
5. sink 节点的执行有3个入口：
   - `PhysicalXxxOperator::Sink(ExecutionContext &context, DataChunk &chunk, OperatorSinkInput &input)`
   - `PhysicalXxxOperator::Combine(ExecutionContext &context, OperatorSinkCombineInput &input)`
   - `PhysicalXxxOperator::Finalize((Pipeline &pipeline, Event &event, ClientContext &context, OperatorSinkFinalizeInput &input)`

在阅读代码过程中，可以通过上述的调试技巧，找到需要学习的代码的入口点，设置断点后，跟着调试器一步一步的阅读代码，并在 Variables 中查看各个变量的
值，理解数据流和代码执行流程。

接下来，我们将从一些简单的SQL 执行示例出发，来阅读 Execution 模块的源代码。
1. [example query: 两个表join后分组聚合](execution-demo1.md)