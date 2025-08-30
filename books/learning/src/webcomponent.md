

# [Why I dont use web components](https://dev.to/richharris/why-i-don-t-use-web-components-2cia)

作者：Rich Harris (svelte 作者)
Date: 2020-7-15

Svelte 支持 Custom Element，并且工作的很好，在 [Custom Elements Everywhere](https://custom-elements-everywhere.com) 上得到了满分，
作者在本文中只是声明他自己为什么不把 WC 作为 svelte 的默认编译选项的一些观点：
1. Progressive Enhancement 渐进式增强
   能够在没有 JavaScript 支持的时候，仍然可以使用。(SSR), svelte 可以很好的 SSR 支持，并且在支持JS的浏览器中，逐步增强。
   > 现在的 Lit 框架对 SSR 的支持仍然有限，考虑到浏览器已经支持 Declarative Shadow DOM，是应该考虑把 SSR 作为 框架的一等公民了。
   > 评论：https://googlechromelabs.github.io/howto-components/howto-tabs/ 兼顾 no-javascript
2. CSS in JS
   作者认为在 JS 中使用字符串形式编写 css，与性能方面的建议相矛盾。未来可以考虑使用  adoptedStyleSheet 和 ::theme, ::part 对
   shadow dom 内部进行样式设置。
3. Platform fatigue
   在2018年这个时间节点，浏览器对 CE 等标准的支持程度还非常的不成熟，而使用 polyfill 也不是一个理想的方案。
   > 2020之后的浏览器都完全支持 v1 规范
4. Polyfills
5. Composition
   作者希望组件能够控制何时渲染其 slot 而非初始化时就进行加载。
   > 需要这种功能时，不应该使用 light dom 而应该延迟加载
6. Confusion between props and attributes
   > 这一点，我没有很理解作者的观点。非 WC 框架，组件只有 props 一个空间，而 WC 有 property + attribute 两个空间，这个谈不上好、或者不好
   > 因为这就是历史的基点。 或许我们要做的，是保持两个空间的一致性，哪些属性应该只在 property 空间，哪些在两个空间，并最好保持一致性。
7. Leaky design
   ```javascript
    const element = document.querySelector('my-thing');
    element.attributeChangedCallback('w', 't', 'f');
   ```
   这或许是个瑕疵, attributeChangeCallback 应该只能是浏览器去调用。
8. The DOM is bad
   作者在这个例子，似乎 Lit 框架中不存在这个问题
9. Global namespace
   所有的Web Component 注册到单个命名空间中
10. These are all solved Problem
   作为认为上面的这些问题，都是曾经存在，并且在框架中已经解决了的，重新发明的WebComponent再次面对这个问题，是一个浪费。

# [Web Components Are Not the Future](https://dev.to/ryansolid/web-components-are-not-the-future-48bh)
