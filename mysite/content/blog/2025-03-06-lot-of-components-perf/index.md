+++
title = "大量组件树下的性能对比"
description = "大量组件树下的性能对比"
date = 2025-03-06
draft = false
template = "blog/page.html"
+++

这个测试是在创建大量组件树的情况下，对比几个前端框架的性能，为在高性能敏感的场景下选择合适的框架提供参考。

## 场景说明：
1. App 包括 1 个 PerfComp20 组件
2. 1个 PerfComp20 组件包括 26 个 PerfComp11 组件
3. 1个 PerfComp11 组件包括 1 个 PerfComp10 组件 (总共 26 个 PerfComp10 组件)
4. 1个 PerfComp10 组件包括 200 个 PerfComp01 组件 （总共 5200 个 PerfComp01 组件）
5. 1个 PerfComp01 组件包括 5 个 PerfComp00 组件 （总共 26000 个 PerfComp00 组件）
6. 整个页面包括：
   - 26 * 200 * 5 = 26000 个 PerfComp00 组件
   - 26 * 200 = 5200 个 PerfComp01 组件
   - 26 = 26 个 PerfComp10 组件
   - 26 = 26 个 PerfComp11 组件
   - 1 = 1 个 PerfComp20 组件
   - 1 = 1 个 App 组件
   - 总共 31254 个组件
   
## 测试结果：

| Part      | Svelte   | Vue3     | Lit      | Lit + Native PerfComp00 | Lit + Native PerfComp00, PerfComp01 | All Native WC |
|-----------|----------|----------|----------|-------------------------|-------------------------------------|---------------|
| Scripting | 134.28ms | 332.95ms | 427.68ms | 265.5ms                 | 112.29ms                            | 85ms          |
| Styling   | 17.39ms  | 15.3ms   | 59.71ms  | 24.86ms                 | 20ms                                | 21ms          |
| Layout    |          | 51.69ms  | 124.53ms | 33ms                    | 31ms                                | 32ms          |
| Paint     | 2.10ms   | 8.6ms    | 34ms     | 20ms                    | 11ms                                | 14ms          | 

## Web Component 的多种方式性能对比
1. Lit Element / Native HTMLComponent
2. DOM structure
   - Shadow DOM + div.text shared style `<div>${text}</div>`
   - Shadow DOM + div.text inlined style `<div style="display: inline;">${text}</div>`
   - Shadow DOM + text `${text}`
   - element DOM + div.text shared style `<div class="inlined">${text}</div>`
   - element DOM + text `${text}`

| Mode   | DOM Structure                       | Scripting | Styling | Layout | Paint |
|--------|-------------------------------------|-----------|---------|--------|-------|
| Lit    | Shadow DOM + div.text shared style  | 470       | 93.70   | 104    | 33    |
| Lit    | Shadow DOM + div.text inlined style | 447       | 46.39   | 105.11 | 33    |
| Lit    | Shadow DOM + text                   | 443       | 26      | 39.27  | 23    |
| Lit    | element DOM + div.text shared style | 428       | 67      | 119    | 31    |
| Lit    | element DOM + text                  | 430       | 27      | 46     | 22    |
| Native | Shadow DOM + div.text shared style  | 103       | 93.97   | 120.85 | 21    |
| Native | Shadow DOM + div.text inlined style | 101       | 42      | 104    | 20    |
| Native | Shadow DOM + text                   | 86        | 20.8    | 32.78  | 14.5  |
| Native | element DOM + div.text shared style | 83        | 43      | 99.5   | 20    |
| Native | element DOM + text                  | 75        | 19      | 31     | 14    |

## 分析结论
1. 基于 Lit 的 Web Component 用于 micro component 时，导致页面中大量的 shadow dom 和 大量的DOM时
   对性能的影响较大，尤其是对 styling 和 layout 的影响的耗时影响较大。
   - scripting: Lit 的初始化开销较大，估计主要是 Lit Template 的开销。按照 [Lit 文档](https://lit.dev/blog/2023-10-10-lit-3.0/#compiler)中的说法，对于 heavy-template 的情况下，采用预编译的
     方式，首次性能提升 45%，更新时提升21%。 对需要大量重复的组件，可以考虑直接使用 Native Web Component。
   - styling: Lit 目前选择的组件级共享的 style，由于在 shadow dom 中引入了 stylesheet，其对性能有一定的影响。相反，Svelte/Vue3 等采用在 document level 的 style 似乎性能更好。
   
     这个与感觉上并不一致，从感觉上来看，shadow dom 内的 style sheet 似乎会因为层次简单而有更高的 match 效率。但实际上，反而有损失，估计与批量处理有关。
   - layout：shadow dom 的引入并不会导致 DOM 的复杂，layout 不会有大的影响。相反，组件带来的 DOM 结构的复杂性，会导致 layout 的性能开销。
   
     对于 Svelte，micro component 自身可能不会引入额外的 DOM 结构，例如直接映射成 document 上的div/text 元素。而 web component 中，每一个 component 自身会
     占用一个 DOM element，在出现大量的组件时，可能会导致 layout 的性能开销。
   
2. Web Component 性能
   - 在本次测试中，使用 Native 编写全部的 Web Component 性能最佳，其开销甚至低于 Svelte（框架开销）。
   - shadow dom 自身并不会影响 DOM 结构的复杂度，不会带来额外的 layout 开销。
   - shadow dom 中的 adoptedStyleSheets 会带来 styling 的开销。当出现大量组件时，Styling 的时间有所增加。
   - 说明引入框架多少会引入开销，尤其是UI组件首次初始化有处理模版、reactive框架等开销。
     在这些方面，Svelte 的优势在于其编译时的静态优化而成本最低。Lit 的 Template compiler 应该是对此看齐，但目前还不成熟。
   
3. 本次测试较为变态，在一般应用场景中如不涉及到如此巨大的组件树（建议一般控制在2000以下时），一般无需担心初始化的性能问题。

## 参考：
1. 项目源代码：[Git Repo](https://github.com/wangzaixiang/lot-of-elements-compare)
2. Recalculate Style
   > In your performance recordings, some of these long-running tasks may be Recalculate Style events. 
   > A Recalculate Style event tracks the time it takes for the browser to do the following:
   > - Iterate through the DOM elements on a page, to find all of the CSS style rules that match a given element.
   > - Compute each element's actual style, based on the matching CSS style rules.
   > 
   > CSS styles need to be recalculated whenever the applicability of CSS rules may have changed, such as:
   > - When elements are added to or removed from the DOM.
   > - When an element's attributes are changed, such as the value of a class or id attribute.
   > - When user input occurs, such as a mouse move or a change of which element in a webpage has focus, which can affect :hover rules. 
   > 
   > see: https://developer.chrome.com/docs/devtools/performance/selector-stats?hl=zh-cn
3. 对大量 DOM 进行更新导致 reflow 时，可以考虑使用 [FastDOM](https://github.com/wilsonpage/fastdom?tab=readme-ov-file)
4. Shadow DOM 对 Style 的性能影响。[中文](https://mp.weixin.qq.com/s?__biz=MzA5NTg1NzI0Mw==&mid=2650000010&idx=1&sn=dd2c9188b1e91467358d542e4ed99001&chksm=895bac7764066c46c70e6fb79023871313717d67db92722402bd04eb296e0735f398e6f393b9#rd) 
   [原文:Does shadow DOM improve style performance?](https://nolanlawson.com/2021/08/15/does-shadow-dom-improve-style-performance/)
   - 如果 page 比较复杂（DOM节点越多、CSS规则越多），shadow dom 可以提高 Style 速度
   - 如果 page 并不复杂，引入 shadow dom 会增加固定的开销，反而增加 Style 开销（从单个大批量转换为多个小批量）
   在很多时候，Style的时间会比 layout 更耗时。
5. [Style scoping versus shadow DOM: which is fastest?](https://nolanlawson.com/2022/06/22/style-scoping-versus-shadow-dom-which-is-fastest/#:~:text=TL%3BDR%3A%20My%20new%20benchmark%20largely%20confirmed%20my%20previous,scenarios%2C%20but%20in%20others%20it’s%20much%20less%20performant)
   总体支持 Shadow DOM 有改善 Style 性能的观点，且性能比较稳定。
   - 相关测试项目 [shadow-selector-benchmark](https://github.com/nolanlawson/shadow-selector-benchmark)
   - 在线测试：https://nolanlawson.github.io/shadow-selector-benchmark/
   - Firefox Stylo 引擎是最快的，几乎可以忽略这一块的开销。在其他浏览器需要花1000ms的场景下，Firefox 只需要花费 100ms。
   - 在2025年，这一块的性能对比 FF(360ms) > Chrome(460ms) > Safari(931ms) Chrome 与 Safari 的差距虽小了。
6. Shadow DOM 对 layout 的性能影响。
   - 减少布局范围，是否仅影响边界内的局部布局？
   - 结合 CSS containment 属性，是否可以减少 layout 的开销？
7. [CSS Containment Performance Example](https://blogs.igalia.com/mrego/files/2019/01/css-contain-example.html)
   使用 contain: strict 带来了8倍的性能提升。在某些程度上，CSS containment 与 shadow dom 有相似之处。

