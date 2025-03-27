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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="preface.html">前言</a></li><li class="chapter-item expanded affix "><li class="part-title">基础流程</li><li class="chapter-item expanded "><a href="parse.html"><strong aria-hidden="true">1.</strong> Parse</a></li><li class="chapter-item expanded "><a href="binder.html"><strong aria-hidden="true">2.</strong> Resolve</a></li><li class="chapter-item expanded "><a href="logical-plan.html"><strong aria-hidden="true">3.</strong> Logical Plan</a></li><li class="chapter-item expanded "><a href="optimizer.html"><strong aria-hidden="true">4.</strong> Optimizer</a></li><li class="chapter-item expanded "><a href="physical-plan.html"><strong aria-hidden="true">5.</strong> Physical Plan</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="vector.html"><strong aria-hidden="true">5.1.</strong> Vector</a></li></ol></li><li class="chapter-item expanded "><a href="execution.html"><strong aria-hidden="true">6.</strong> Execution</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="execution-demo1.html"><strong aria-hidden="true">6.1.</strong> execution-demo1</a></li><li class="chapter-item expanded "><a href="pipeline.html"><strong aria-hidden="true">6.2.</strong> Pipeline</a></li></ol></li><li class="chapter-item expanded "><a href="operators.html"><strong aria-hidden="true">7.</strong> Operators</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="seq_scan.html"><strong aria-hidden="true">7.1.</strong> seq_scan</a></li><li class="chapter-item expanded "><a href="filter.html"><strong aria-hidden="true">7.2.</strong> filter</a></li><li class="chapter-item expanded "><a href="projection.html"><strong aria-hidden="true">7.3.</strong> projection</a></li><li class="chapter-item expanded "><a href="projection.html"><strong aria-hidden="true">7.4.</strong> projection</a></li><li class="chapter-item expanded "><a href="hash_join.html"><strong aria-hidden="true">7.5.</strong> hash_join</a></li><li class="chapter-item expanded "><a href="hash_group_by.html"><strong aria-hidden="true">7.6.</strong> hash_group_by</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">其他组件</li><li class="chapter-item expanded "><a href="storage.html"><strong aria-hidden="true">8.</strong> Storage</a></li><li class="chapter-item expanded "><a href="cli.html"><strong aria-hidden="true">9.</strong> CLI</a></li><li class="chapter-item expanded "><a href="read_csv.html"><strong aria-hidden="true">10.</strong> read_csv 表函数分析</a></li><li class="chapter-item expanded affix "><li class="part-title">扩展</li><li class="chapter-item expanded "><a href="extensions.html"><strong aria-hidden="true">11.</strong> Extensions</a></li><li class="chapter-item expanded affix "><li class="part-title">性能优化</li><li class="chapter-item expanded affix "><li class="part-title">OLAP.NEXT 思考</li><li class="chapter-item expanded "><a href="ir.html"><strong aria-hidden="true">12.</strong> Typed IR</a></li><li class="chapter-item expanded "><a href="subquery-opt.html"><strong aria-hidden="true">13.</strong> subquery optimization</a></li><li class="chapter-item expanded "><a href="next-misc.html"><strong aria-hidden="true">14.</strong> Misc</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
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
