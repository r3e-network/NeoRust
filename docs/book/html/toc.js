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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><a href="index.html">Introduction</a></li><li class="chapter-item affix "><li class="part-title">User Guide</li><li class="chapter-item "><a href="guides/getting-started.html"><strong aria-hidden="true">1.</strong> Getting Started</a></li><li class="chapter-item "><a href="guides/installation.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item affix "><li class="part-title">Tutorials</li><li class="chapter-item "><a href="tutorials/wallet-management.html"><strong aria-hidden="true">3.</strong> Wallet Management</a></li><li class="chapter-item "><a href="tutorials/smart-contracts.html"><strong aria-hidden="true">4.</strong> Smart Contracts</a></li><li class="chapter-item "><a href="tutorials/transactions.html"><strong aria-hidden="true">5.</strong> Transactions</a></li><li class="chapter-item "><a href="tutorials/nep17-tokens.html"><strong aria-hidden="true">6.</strong> NEP-17 Tokens</a></li><li class="chapter-item "><a href="tutorials/nns.html"><strong aria-hidden="true">7.</strong> Neo Name Service</a></li><li class="chapter-item "><a href="tutorials/neo-x.html"><strong aria-hidden="true">8.</strong> Neo X Integration</a></li><li class="chapter-item "><a href="tutorials/sgx.html"><strong aria-hidden="true">9.</strong> SGX Support</a></li><li class="chapter-item affix "><li class="part-title">API Reference</li><li class="chapter-item "><a href="reference/api-overview.html"><strong aria-hidden="true">10.</strong> API Overview</a></li><li class="chapter-item "><a href="reference/configuration.html"><strong aria-hidden="true">11.</strong> Configuration</a></li><li class="chapter-item "><a href="reference/error-handling.html"><strong aria-hidden="true">12.</strong> Error Handling</a></li><li class="chapter-item "><a href="reference/copyright.html"><strong aria-hidden="true">13.</strong> Copyright Information</a></li><li class="chapter-item affix "><li class="part-title">Cryptography</li><li class="chapter-item "><a href="tutorials/wallet-management.html"><strong aria-hidden="true">14.</strong> Cryptographic Operations</a></li><li class="chapter-item affix "><li class="part-title">Examples</li><li class="chapter-item "><a href="examples/index.html"><strong aria-hidden="true">15.</strong> Example Code</a></li></ol>';
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
