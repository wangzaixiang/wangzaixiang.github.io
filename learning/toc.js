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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">C++</li><li class="chapter-item expanded "><a href="cpp_optimizations.html"><strong aria-hidden="true">1.</strong> CPP Optimizations</a></li><li class="chapter-item expanded affix "><li class="part-title">rust-lang</li><li class="chapter-item expanded "><a href="rust_prefaces.html"><strong aria-hidden="true">2.</strong> Prefaces</a></li><li class="chapter-item expanded "><a href="rust_vectorization-1.html"><strong aria-hidden="true">3.</strong> compiler vectorization</a></li><li class="chapter-item expanded "><a href="rust_sugars.html"><strong aria-hidden="true">4.</strong> rust sugars</a></li><li class="chapter-item expanded "><a href="rust_type_system.html"><strong aria-hidden="true">5.</strong> rust 类型系统</a></li><li class="chapter-item expanded "><a href="rust_unsafe_cast.html"><strong aria-hidden="true">6.</strong> unsafe: cast &amp;T to &amp;mut T</a></li><li class="chapter-item expanded "><a href="rust-2025-lifecycle.html"><strong aria-hidden="true">7.</strong> lifecycle in rust 2025</a></li><li class="chapter-item expanded "><a href="rust-rc-layout.html"><strong aria-hidden="true">8.</strong> Rc layout</a></li><li class="chapter-item expanded affix "><li class="part-title">simd</li><li class="chapter-item expanded "><a href="simd-1.html"><strong aria-hidden="true">9.</strong> simd-1</a></li><li class="chapter-item expanded affix "><li class="part-title">LLVM</li><li class="chapter-item expanded "><a href="demo-llvm-ir.html"><strong aria-hidden="true">10.</strong> 从一个简单的C代码来学习LLVM-IR</a></li><li class="chapter-item expanded affix "><li class="part-title">QBE</li><li class="chapter-item expanded "><a href="qbe-1.html"><strong aria-hidden="true">11.</strong> 初始QBE</a></li><li class="chapter-item expanded "><a href="qbe-core-concepts.html"><strong aria-hidden="true">12.</strong> QBE core concepts</a></li><li class="chapter-item expanded "><a href="qbe-data-structure.html"><strong aria-hidden="true">13.</strong> QBE 核心数据结构</a></li><li class="chapter-item expanded "><a href="qbe-2.html"><strong aria-hidden="true">14.</strong> QBE 源代码阅读 1</a></li><li class="chapter-item expanded "><a href="qbe-fillrpo.html"><strong aria-hidden="true">15.</strong> QBE 源代码阅读 2: fillrpo</a></li><li class="chapter-item expanded "><a href="qbe-fillpreds.html"><strong aria-hidden="true">16.</strong> QBE 源代码阅读 3: fillrpo</a></li><li class="chapter-item expanded "><a href="qbe-promote.html"><strong aria-hidden="true">17.</strong> QBE 源代码阅读 4：promote</a></li><li class="chapter-item expanded affix "><li class="part-title">zig-lang</li><li class="chapter-item expanded "><a href="zig_misc.html"><strong aria-hidden="true">18.</strong> misc</a></li><li class="chapter-item expanded "><a href="zig_print_in_zig.html"><strong aria-hidden="true">19.</strong> understand print in zig</a></li><li class="chapter-item expanded "><a href="zig_comptime.html"><strong aria-hidden="true">20.</strong> comptime</a></li><li class="chapter-item expanded "><a href="zig_how_comptime_works.html"><strong aria-hidden="true">21.</strong> How comptime works</a></li><li class="chapter-item expanded "><a href="zig_stack_layout.html"><strong aria-hidden="true">22.</strong> Stack-Layout</a></li><li class="chapter-item expanded "><a href="zig_a_bug.html"><strong aria-hidden="true">23.</strong> A Zig Compiler Bug</a></li><li class="chapter-item expanded "><a href="zig_dynamic_construct_a_type_in_comptime.html"><strong aria-hidden="true">24.</strong> dynamic construct a type in comptime</a></li><li class="chapter-item expanded "><a href="zig_soa_test.html"><strong aria-hidden="true">25.</strong> Zig Structure of Array test</a></li><li class="chapter-item expanded affix "><li class="part-title">scala-lang</li><li class="chapter-item expanded affix "><li class="part-title">arrow + datafusion</li><li class="chapter-item expanded "><a href="arrow_array.html"><strong aria-hidden="true">26.</strong> Array</a></li><li class="chapter-item expanded "><a href="df-case-1.html"><strong aria-hidden="true">27.</strong> df-case-1</a></li><li class="chapter-item expanded "><a href="perf-vs-duckdb-1.html"><strong aria-hidden="true">28.</strong> compare-with-duckdb</a></li><li class="chapter-item expanded "><a href="sql-join.html"><strong aria-hidden="true">29.</strong> Join Performance</a></li><li class="chapter-item expanded "><a href="datafusion/hash-join.html"><strong aria-hidden="true">30.</strong> Hashjoin</a></li><li class="chapter-item expanded "><a href="datafusion/window_function.html"><strong aria-hidden="true">31.</strong> window function</a></li><li class="chapter-item expanded "><a href="datafusion-misc.html"><strong aria-hidden="true">32.</strong> datafusion-misc</a></li><li class="chapter-item expanded affix "><li class="part-title">OLAP</li><li class="chapter-item expanded "><a href="olap_cube_js.html"><strong aria-hidden="true">33.</strong> cube.js</a></li><li class="chapter-item expanded affix "><li class="part-title">AI</li><li class="chapter-item expanded "><a href="llm/handon-llm.html"><strong aria-hidden="true">34.</strong> 图解大模型：生成式AI原理与实践</a></li><li class="chapter-item expanded affix "><li class="part-title">Misc</li><li class="chapter-item expanded "><a href="webcomponent.html"><strong aria-hidden="true">35.</strong> webcomponent</a></li></ol>';
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
