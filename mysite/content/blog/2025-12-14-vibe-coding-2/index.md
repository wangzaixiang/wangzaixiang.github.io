+++
title = "Vibe Coding 实践记录"
description = "本文记录最近使用 vibe coding 的一些实战记录"
data = "2025-12-14"
dfaft = false
template = "blog/page.html"

[extra]
toc = true
+++

# 背景

最近一段时间，比较重度的使用 AI 开发了2个软件，都还是蛮有挑战性的：

## molap storage

说是 MOLAP(Multi-dimension Online Analysis Process) storage，这个名字起得大了一些，其实本项目只是针对 MOLAP 计算中的计算度量的预计算
的存储。一般的 MOLAP 引擎紧处理原子度量的预计算的存储，因为原子度量自身具备再聚合的能力，其存储的性价比相对较高，而且其本身对计算度量的计算
也是具有加速效果的。而大部份的计算度量，不具有再计算性。

本质而言，这个存储引擎就是一个简单的 Key-Value 存储引擎，其基本的 API 是：
```rust
trait DB {
    fn put(&self, cube_id: u16, dimensions: &[&str], value: Value) -> Result<()>;
    fn get(&self, cube_id: u16, dimension: &[&str]) -> Result<Value>;
}
```
不过，简单的 API 背后，蕴含了不少复杂的设计挑战：
- 高效的存储格式设计，作为预计算的存储引擎，而不仅仅是存放到内存中。
- 高效的读写性能，尤其是读性能，在我最早的设计目标中，希望能够达到 10M QPS 的读性能。(单个请求耗时 ~100ns)，这里需要尽可能的匹配现代 CPU 的特性，
  包括缓存有好、SIMD 指令集的使用等。
- 高效的内存使用。这个 API 将在 JVM 中使用，如果需要占用过多的 JVM Heap 内存，将会影响到整体的 JVM 性能。
- 零启动开销。即使对一次性的脚本任务，也希望能够快速启动，避免因为存储引擎的初始化而带来过多的延迟。设计目标是小于 1ms 的启动时间（读），
  一个重大的挑战就是可直接 mmap 的数据结构，避免任何的数据序列化操作和数据复制开销。

在[the 1brc program](@/blog/2025-02-06-the-1brc-program/index.md) 项目中的极致性能挑战，为 molap storage 的设计提供了很好的经验积累,
在这项目的开发过程中，我基本上没有花费太大的代价，只经历了几轮快速调优后，就达到了性能上的极致目标。

上述的设计目标，一开始就注定这个项目是极具挑战性的，也能充分发挥 Rust 的顶层编程和高性能的挑战。

## reactive-system
这个项目是一个响应式系统引擎，是我规划中的 Analysis Report(一个类似于 Observable Notebook 的数据分析工具)的核心引擎。这个引擎的设计目标是：
- 支持异步的响应式计算。
- 为交互式 notebook 提供高效的响应式计算引擎。
- 支持复杂的响应式计算图，支持动态的计算图结构。
- 保守保证计算的因果一致性，绝不出现“时间倒流”、“数据不一致”的现象。
- 激进的调度优化：严格的拓扑排序下的调度，避免重复计算，并且在因为变化而导致的计算过期时，能够及时取消过期计算，节省计算资源。

24年，我设计过 reactive-system 的第一个版本，并且在公司的交互式仪表盘中应用，对解决交互式数据分析中的数据依赖、数据联动等问题，发挥了重要的
作用（这一块也是产品历史BUG的重灾区）。在 V1 的设计基础上，我为响应式系统进行引入了新的目标：
- 支持 notebook 中 code cell 这样的 多输入，多输出的计算单元（V1 是多输入，单输出）。
- 定义了更保守的因果一致性模型，确保在异步计算环境下，数据的一致性和正确性。
- 引入了更激进的调度优化策略，对任何过时的计算任务，能够及时取消，节省计算资源。
- 更明确的异常传播机制。将异常作为计算结果的一部分进行传播，确保下游计算能够正确处理上游的异常情况。

这个系统的核心 API 很简单：
```typescript
interface ReativeModule {
    defineSource(source: { id: string, initialValue?: any}): void;
    defineComputation(computation: {
        id: string;
        inputIds: string[];
        outputIds: string[];
        body: (scope: Scope, signal: AbortSignal) => Promise<Record<string, any>>;
    }): void;
    updateSource(id: string, newValue: any): void;
    getValue(id: string): Promise<any>;
    observe(id: string, observer: (result: Result<any>) => void): Unsubscribe;
}
interface Scope {
    [variableId: string]: Promise<any>;
    __getResult: (variableId: string) => Promise<Result<any>>;
}
type Unsubscribe = () => void;

```

当然，24年我仅是作为设计者，而开发是由一位前端同事完成的，而且，彼时并没有借助 AI 的力量，开发过程相对较长，而且也是在开发的过程中不算确认设计
细节。而现在，则是由我来亲自设计、编码，并且是借助 Claude Code /Gemini 等 AI 助力来完成开发任务，这个过程也想比传统的开发方式有了很大的不同，
在差不多2周的时间里，我其实是完成了2版大型的设计改造（从 V2 的 MVCC 设计，到 V3 的基于 cause_at 的时间因果设计），在第3版中，又进行多次的
实现细节的重大调整，对调度、取消机制、状态管理等都做了好几次的重构，几乎是每天一次大型的代码重构），最终完成了 V3 的设计和实现，达到了我心目中
（暂时）比较完美的感觉。

---

简单来说，这两个系统都比较类似于 《A Philosophy of Software Design》一书中的原则：
- 更窄的接口：接口提供了高度简单、抽象的语义，隐藏了复杂的实现细节。
- 更深的实现：在简单的接口之后，隐藏了复杂的实现细节，这些细节都是偏重于性能、一致性的机制优化，但并不暴露使用的复杂性。

# workflow: feature-phase-task workflow
在这两个项目的开发过程中，我逐步形成了我自己的一套 workflow，非常适合于 vibe coding 的开发模式，相关的文档可以参考：
- [feature-phase-task workflow](@/blog/2025-11-01-feature-phase-task-workflow/index.md)

使用这个 workflow 优点：
- 贴合 Spec First 的模式：我们主要的精力在于需求规格和设计文档的准备上（当然这个过程 AI 可以给你很多的助力，但主要的职责是你）
- 在 Spec 准备好之后，我们可以先和 AI 一起评估一个开发的计划：分解成为多个 phase,每个 phase 包含多个 task。
- 对于不确定比较高的系统，我们需要启动 phase 0 阶段：通过定义end-to-end 的使用场景，基于这些使用场景，编写测试用例，确定整体的 API 设计，
  在确定了 API 设计之后，再进行后续的 phase 拆解和 task 的拆解。
- feature 对应于 Feature Driven Development 的 feature，或者 git flow 中的一个 feature 分支，一个 Pull Request。是一个完整的功能特性。
  多个 feature 可以并行进行开发，最后 merge 到主分支。
- phase 对应于一个小的迭代目标，一个 phase 可以用于验证某个原型，为下一个 phase 做准备，可以是丢弃型的原型开发，也可以是增量式的开发。
- task 对应于一个具体的开发任务，可以是一个小的功能点，或者一个模块的实现。task 是最小的开发单元。
- 完成规划后，就可以以 AI 为主的方式去进行推进，在开发的过程中，适当的进行干预即可，而无需高频率的进行讨论和编码沟通。这也使得我们可以并行进行多个
  feature 的开发。

# AI 的 长项 与 短板
未完，待续

# 大刀阔斧的重构
未完，待续