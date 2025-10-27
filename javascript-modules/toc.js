// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">基础概念</li><li class="chapter-item expanded "><a href="history.html"><strong aria-hidden="true">1.</strong> 模块化发展历史</a></li><li class="chapter-item expanded "><a href="why-modules.html"><strong aria-hidden="true">2.</strong> 为什么需要模块化</a></li><li class="chapter-item expanded "><a href="concepts.html"><strong aria-hidden="true">3.</strong> 模块化的核心概念</a></li><li class="chapter-item expanded affix "><li class="part-title">ES模块系统 (ESM)</li><li class="chapter-item expanded "><a href="esm/basics.html"><strong aria-hidden="true">4.</strong> ES模块基础</a></li><li class="chapter-item expanded "><a href="esm/import-export.html"><strong aria-hidden="true">5.</strong> 导入与导出</a></li><li class="chapter-item expanded "><a href="esm/dynamic-import.html"><strong aria-hidden="true">6.</strong> 动态导入</a></li><li class="chapter-item expanded "><a href="esm/resolution.html"><strong aria-hidden="true">7.</strong> 模块解析机制</a></li><li class="chapter-item expanded "><a href="esm/hot-module-reload.html"><strong aria-hidden="true">8.</strong> 热模块重载</a></li><li class="chapter-item expanded "><a href="esm/circular-deps.html"><strong aria-hidden="true">9.</strong> 循环依赖处理</a></li><li class="chapter-item expanded affix "><li class="part-title">CommonJS模块系统</li><li class="chapter-item expanded "><a href="cjs/basics.html"><strong aria-hidden="true">10.</strong> CommonJS基础</a></li><li class="chapter-item expanded "><a href="cjs/require-exports.html"><strong aria-hidden="true">11.</strong> require与module.exports</a></li><li class="chapter-item expanded "><a href="cjs/caching.html"><strong aria-hidden="true">12.</strong> 模块缓存机制</a></li><li class="chapter-item expanded "><a href="cjs/interop.html"><strong aria-hidden="true">13.</strong> 与ES模块的互操作</a></li><li class="chapter-item expanded affix "><li class="part-title">AMD与UMD</li><li class="chapter-item expanded "><a href="amd/basics.html"><strong aria-hidden="true">14.</strong> AMD规范</a></li><li class="chapter-item expanded "><a href="amd/requirejs.html"><strong aria-hidden="true">15.</strong> RequireJS实践</a></li><li class="chapter-item expanded "><a href="umd/universal.html"><strong aria-hidden="true">16.</strong> UMD通用模块</a></li><li class="chapter-item expanded affix "><li class="part-title">现代工具链</li><li class="chapter-item expanded "><a href="tooling/bundlers.html"><strong aria-hidden="true">17.</strong> 打包工具</a></li><li class="chapter-item expanded "><a href="tooling/transpilers.html"><strong aria-hidden="true">18.</strong> 转译工具</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tooling/babel.html"><strong aria-hidden="true">18.1.</strong> Babel模块转换</a></li><li class="chapter-item expanded "><a href="tooling/typescript.html"><strong aria-hidden="true">18.2.</strong> TypeScript模块</a></li></ol></li><li class="chapter-item expanded "><a href="tooling/no-build.html"><strong aria-hidden="true">19.</strong> 无构建开发</a></li><li class="chapter-item expanded affix "><li class="part-title">运行环境差异</li><li class="chapter-item expanded "><a href="runtime/browser.html"><strong aria-hidden="true">20.</strong> 浏览器中的模块</a></li><li class="chapter-item expanded "><a href="runtime/nodejs.html"><strong aria-hidden="true">21.</strong> Node.js中的模块</a></li><li class="chapter-item expanded "><a href="runtime/deno.html"><strong aria-hidden="true">22.</strong> Deno的模块系统</a></li><li class="chapter-item expanded "><a href="runtime/bun.html"><strong aria-hidden="true">23.</strong> Bun的模块系统</a></li><li class="chapter-item expanded affix "><li class="part-title">最佳实践</li><li class="chapter-item expanded "><a href="best-practices/design.html"><strong aria-hidden="true">24.</strong> 模块设计原则</a></li><li class="chapter-item expanded "><a href="best-practices/performance.html"><strong aria-hidden="true">25.</strong> 性能优化</a></li><li class="chapter-item expanded "><a href="best-practices/error-handling.html"><strong aria-hidden="true">26.</strong> 错误处理</a></li><li class="chapter-item expanded "><a href="best-practices/testing.html"><strong aria-hidden="true">27.</strong> 测试模块化代码</a></li><li class="chapter-item expanded affix "><li class="part-title">高级主题</li><li class="chapter-item expanded "><a href="advanced/microfrontends.html"><strong aria-hidden="true">28.</strong> 微前端模块化</a></li><li class="chapter-item expanded "><a href="advanced/web-workers.html"><strong aria-hidden="true">29.</strong> Web Workers中的模块</a></li><li class="chapter-item expanded "><a href="advanced/wasm.html"><strong aria-hidden="true">30.</strong> WASM模块集成</a></li><li class="chapter-item expanded affix "><li class="part-title">实战案例</li><li class="chapter-item expanded "><a href="examples/library.html"><strong aria-hidden="true">31.</strong> 构建一个模块化库</a></li><li class="chapter-item expanded "><a href="examples/large-project.html"><strong aria-hidden="true">32.</strong> 大型项目模块组织</a></li><li class="chapter-item expanded "><a href="examples/lazy-loading.html"><strong aria-hidden="true">33.</strong> 模块懒加载实现</a></li><li class="chapter-item expanded affix "><li class="part-title">附录</li><li class="chapter-item expanded "><a href="appendix/tools-comparison.html"><strong aria-hidden="true">34.</strong> 模块相关工具对比</a></li><li class="chapter-item expanded "><a href="appendix/faq.html"><strong aria-hidden="true">35.</strong> 常见问题解答</a></li><li class="chapter-item expanded "><a href="appendix/references.html"><strong aria-hidden="true">36.</strong> 参考资源</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
