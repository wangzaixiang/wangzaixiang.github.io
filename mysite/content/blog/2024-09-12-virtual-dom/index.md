+++
title = "DOM 更新技术对比：Virtual DOM or Incremental DOM"
description = "WEB 前端开发中，DOM 的增量更新技术是一个重要的技术，本文对比了 Virtual DOM 和其他技术的优缺点。"
date = 2024-09-12
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["wangzx"]

[extra]
toc = true
+++

DOM 更新技术本身是一步技术演进历史，本文收集了从远古时间 到 最新的技术，并整理了我对其的理解。目前还只是一个提纲，后续有时间会慢慢细化。

1. 直接、命令式操作 DOM 时代
   
   命令式编程的时代，也是 UI 的传统时代，无比黑暗、无趣的时代。从 Motif 到 Win32，都是这种风格的编程模式。
   - innerHTML 操作：简单、粗暴
   - 诞生了 jquery 这一王者
2. React 时代: ui = f(state)
   
   进入到了函数式编程的时代，不仅WEB，桌面 GUI 也开始以 React 风格为主，如 Flutter, Slint 等。
   1. React Virtual DOM
      ```javascript
      function render() {
         let vdom = f(state)
         let delta = diff(dom, vdom)
         patch(dom, delta)
      }
      // Loop: event -> change state -> render 
      ```
   2. incremental DOM
      ```javascript
      function render() {
         let delta = f(state, dom) // compute virtual dom side by side dom
         patch(dom, delta)
      }
      // Loop: event -> change state -> render 
      ```   
      通过在计算 virtual dom 时，即与 DOM 进行比较，由于在大部份情况下，变更紧紧是一小部份，因此，可以减少内存的占用。这种模式感觉应该
      是对 virtual dom 的一种有效优化，但在实际生态中，好像并没有太多的实现。
   3. Lit-Style
      ```javascript
      function render() {
         let [static_parts, [position, state]] = f(state); // 得益于 lit template
         let old_state = dom[position];  // dynamic 部份会有多个
         if(old_state != state) {
            update(position, state);
         }
      } 
      ```
      Lit 基于 template，由于可以识别 template 的静态部份（不会变化）和动态部份（会响应数据的变化），
      Lit 在 render 后，会追踪动态部份，检查动态部份依赖的数据是否发生变化，做出响应的更新。（参见很早期我编写的一篇文章：
      [LitElement & LitHtml 探秘](/blog/litelement) )
      缺点：每次 render 函数调用都会执行，逐一检查动态部份是否发生变化，这个过程还是略显多余。
      优点：算是一种相对简单的平衡，即没有大量的 virtual dom 的内存占用(static部份是共享的，不会重复占用内存)，
      也简化了 DOM 的 diff过程。
   4. Solid-Style/Svelte-Style
      ```javascript
      function render(){
        let dom = f(state);
        createEffect( () => { // 对每一个动态部份，追踪其依赖的数据
           dom1 = f1(state1);  // 当子状态发生后变化时，更新 dom1
        } );
      }
      ```  
      这一类的框架，在 Lit 的基础上更进一步：在后续响应式变化时，render 函数不再执行，而是每个动态部份独立响应。当然，做到这一点是
      依赖于对模版的编译的。即将模版中的动态部份、静态部份在编译期即进行了分离，并生成对应的响应式代码。
      当然，对一些复杂的结构，需要进一步展开其响应式机制。
   5. 未来发展预测
      在编译期进行更多的优化，从而避免在运行期做不必要的计算，这个方向应该还有不少空间，例如：诸如 Solid 目前设计（我的猜测），还是采用
      动态的 track 方式来跟踪变化的依赖，在依赖发生变化时，重新计算，从重新刷新依赖关系。这个对于一些静态的依赖关系，可以在编译期直接确定，
      进而减少 track 的开销。
   
  最新的发展，这种 React 风格也逐步的占领了桌面 GUI 的领域，如 Flutter, Slint 等。在最近留意的一个框架 dioxuslabs 中，更是把
  这个带入到 Rust + WASM + (Web/TUI/GUI)的风格中，令 Rustacean 们也有了一种新的选择。

# Links
1. React
   [Rethinking Best Practices](https://www.slideshare.net/slideshow/react-preso-v2/26589373)
2. Svelte 系列
    1. [Virtual DOM is pure overhead](https://svelte.dev/blog/virtual-dom-is-pure-overhead)
    2. [Rethinking reactivity](https://svelte.dev/blog/svelte-3-rethinking-reactivity)
    3. [Rethinking 'Rethinking reactivity'](https://svelte.dev/blog/runes)
3. SolidJS
   1. [A Hands-on Introduction to Fine-Grained Reactivity](https://dev.to/ryansolid/a-hands-on-introduction-to-fine-grained-reactivity-3ndf)
      我设计的 Variable Manager 设计方案，在很多特性上，与这篇文章的设计思路是一致的。包括：
      - dynamic dependency tracking。memoization 和 effect 都类似于VM 中的 binding。
      - batched updates. 类似于 VM 中的 transaction。
      只是，本文更为微观，面向 synchronous，而 vm 更为宏观，面向 asynchronous。
      只是，Solid的处理中应该使用了 weak reference，当 effect 回收时，这个计算也自动销毁了。
   2. [Building a Reactive Library from Scratch](https://dev.to/ryansolid/building-a-reactive-library-from-scratch-1i0p)
   3. [SolidJS: Reactivity to Rendering](https://angularindepth.com/posts/1289/solidjs-reactivity-to-rendering)
      1. example1
         ```javascript
         const Greeting = (props) => (
             <>Hi <span>{props.name}</span></>
         );
         const App(() => {
            const [visible, setVisible] = createSignal(false),
            [name, setName] = createSignal("Josephine");

            return (
               <div onClick={() => setName("Geraldine")}>{
               visible() && <Greeting name={ name } />
               }</div>
            );
         });
         ```
         等效于代码:
         ```javascript
         const Greeting = (props) => {
             let t1 = Text();
             [name, setName] = null; // signal from props.
             createEffect(() => {
                t1.data = $name;
             });
             return [ Text("Hi "), Span(t1) ];
         };
         
         const App = () => {
            let els = computeEffect( ()=>{
                if( visble() ){
                     let el = Greeting();
                     computeEffect(() => {
                        el.propSet("name", name());
                     });
                     return [ el ];
                }
                else {
                   return [];
                }
            })
            { div {
               ...els 
              } 
            }
         }
         ```
      2. 
4. Angular
   1. [Change Detection Big Picture - overview](https://angularindepth.com/posts/300/big-picture-overview)
   2. [Change Detection Big Picture - Operations](https://angularindepth.com/posts/301/big-picture-operations)
   3. [Change Detection Big Picture - Unidirectional data flow](https://angularindepth.com/posts/302/big-picture-unidirectional-data-flow)
   4. [Change Detection Big Picture - Rendering cycle](https://angularindepth.com/posts/303/big-picture-rendering-cycle)
   5. [Change Detection Big Picture - Components tree](https://angularindepth.com/posts/304/running-change-detection-components-tree)
5. Lit 的做法
6. [增量DOM与虚拟DOM](https://www.cnblogs.com/zhazhanitian/p/14421993.html)
7. Incremental-DOM https://google.github.io/incremental-dom/
8. [Xilem: an architecture for UI in Rust](https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html) 

   Xilem 是一个很类似于 Flutter/SwiftUI 的 Rust GUI 框架，其核心也是一个增量更新的架构，在 [Xilem架构](https://github.com/linebender/xilem/blob/main/ARCHITECTURE.md)
   中有很多这方面的思考，可以作为对 UI 的增量更新的参考。 