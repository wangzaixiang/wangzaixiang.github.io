+++
title = "Nov 2025"
description = "2025-11 月记"
date = 2025-11-12
draft = false
template = "blog/page.html"
+++

# Languages
## Rust
1. [Inside Cargo’s Parallel Compilation Engine: How Rust Builds Faster Than You Think](https://medium.com/@bugsybits/inside-cargos-parallel-compilation-engine-how-rust-builds-faster-than-you-think-2515bed4f604)
  了解 Cargo 和 rustc 的编译过程
  - DAG of crates：依据 crate之间的依赖关系，无需依赖源代码分析。
  - the new parallel front-end: parsing, HIR, MIR, type-checking, borrow-checking
    - crate as a unit, serial  
  - Code Generation Units: LLVM IR generation, optimization, codegen.
    - a crate may split more CGUs in parallel threads  
  - Linking, often a bottleneck.
  - Incremental & Caching: only changed crates or units are rebuilt, metadata(.rmeta) files help.
  ![img_1.png](img_1.png)  
  借助与 codex，我生成了一个流程图：
  {% mermaid() %}
  ```mermaid
  graph TD
    A["Source files"] --> B["Parse & macro expand"]
    B --> C["HIR build & fingerprint"]
    C --> D["Type check & borrow check\n[cache]"]
    D --> E["MIR build & opt passes\n[cache]"]

    E --> F{{"Split codegen units\n(parallel)"}}
    F --> G1["MIR → LLVM IR (per CGU)\n[cache]"]
    G1 --> H1["LLVM opt (per CGU)\n[cache]"]
    H1 --> I1["Codegen to object/bitcode (.o/.bc)\n[cache]"]

    I1 --> L{"LTO enabled?"}
    L -- "No" --> J{{"Join objects"}}
    L -- "ThinLTO" --> T1{{"ThinLTO index & schedule\n(parallel)"}}
    T1 --> T2["ThinLTO cross-unit opt/codegen\n[cache]"]
    T2 --> J
    L -- "Full LTO" --> F1["Monolithic LTO pipeline\n[cache]"]
    F1 --> J

    J --> K["Link final artifact"]

    classDef cache fill:#e8f5e9,stroke:#388e3c,color:#1b5e20;
    classDef parallel fill:#e3f2fd,stroke:#1e88e5,color:#0d47a1;

    class D,E,G1,H1,I1,T2,F1 cache;
    class F,J,T1 parallel;
  ```
  {% end %}
 
## Scala
1. Scala 3.7.4 released. 

# Mpp & OLAP

# Web & Frontend
1. [juris.js](https://jurisjs.com) 又一个组件框架
   - no build
   - 使用 JSON-Like 描述 DOM (JavaScript Object Notion): 
     - static value: no reactive
     - function: reactive
     - enhance(): 逐步增强模式：对静态 HTML 进行增强
   > 是否需要一种更好的 structure object notion? 支持更多的数据类型？还是 JSON ? 可以与我之前设计的 XUL 进行一个对比。
   > 部分的设计思路是值得参考的。 
   > 感觉：
   > - 新概念不多，比较简洁。
   > - 由于没有 build 阶段，难以实现诸如 solidjs/svelte 这类的精细化响应式
   > - 作为 DSL 的简洁性值得商榷。 
2. [Juris Performance Compare](https://medium.com/@resti.guay/your-framework-vs-juris-js-on-data-table-rendering-ff3920b2316e)
   这篇文章对比了 Juris/Svelte/React/Vue3/Angular 等框架在一个案例：100 table rows with 4 actions and 5 editable cells 上的
   内存占用、渲染耗时、事件延迟方面的对比数据。可以作为对框架性能对比的一个参考。
   ![img.png](img.png)
   - 这篇文章中的 Juris 相比 Svelte 在 render time 上还更快速，batch rendering mode? 这个原因不确定。
   - 事件的延迟也比 Svelte 更快，Why?
   
   > 关注了众多的 Web 组件框架，我心目中理想的 组件框架 长什么模样？后续需要花些时间思考并整理：
   > - build or no-build
   > - V-DOM or DOM
   > - 精确响应式？
   > - JSON or JSX or ...

3. [Typst Studio in Pure Rust: WebAssembly and Rust for Modern Web ApplicationswTypst Studio in Pure Rust: WebAssembly and Rust for Modern Web Applicationstudio](https://autognosi.medium.com/typst-studio-in-pure-rust-webassembly-and-rust-for-modern-web-applications-4e2e52be14a2)
   - [Typst Studio](https://automataia.github.io/wasm-typst-studio-rs/)
   > WASM 的最佳应用场景在哪里？
   > Typst 是否适合作为 markdown 的替代？作为数据分析和可视化领域内的文档载体呢？ 

# AI & Agent
1. [toon](https://github.com/toon-format/toon) token-oriented object notion
   LLM 时代的 JSON/YML 替代， 相比 JSON 节省40% token，准确性还更高。
2. Graph RAG
   对数据分析类的 agent, Graph RAG 可以做些什么？

# Misc
