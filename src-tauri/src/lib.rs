#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|window, _payload| {
            // window.open_devtools(); // force open  
            window.eval(
                    r##"
                console.log("Minsta Runninggg!!!!!!!")
                // CSS injection
                setInterval(() => {
                    if (!document.querySelector('style[data-minsta]')) {
                        const s = document.createElement('style');
                        s.setAttribute('data-minsta', '1');
                        s.textContent = `
                            a[href="/reels/"] { display: none !important; }
                            a[href="/explore/"] { display: none !important; }
                            a[href="#"] { display: none !important; }
                            // article { display: none !important; }
                        `;
                        document.head.appendChild(s);
                    }
                }, 500);
                // Only Hide Suggested
                function getTargetDiv(suggested) {
                    let el = suggested;
                    let steps = 0;
                    const MAX_STEPS = 9;

                    while (el && steps < MAX_STEPS) {
                        el = el.parentElement;
                        if (!el) break;

                        // ---- condition 1 ----
                        if (el.tagName === "DIV") {
                        const prev = el.previousElementSibling;
                        const next = el.nextElementSibling;

                        if (
                            (prev && prev.tagName === "ARTICLE") ||
                            (next && next.tagName === "ARTICLE")
                        ) {
                            return el;
                        }
                        }

                        steps++;
                    }

                    // ---- condition 2 (fallback) ----
                    el = suggested;
                    steps = 0;

                    while (el && steps < MAX_STEPS) {
                        el = el.parentElement;
                        if (!el) break;

                        if (el.tagName === "DIV" && el.classList.length === 0) {
                        return el;
                        }

                        steps++;
                    }

                    return null;
                }
                function hideSuggestedAndBelow() {
                    console.log("Hiding Below")
                    const allSpans = document.querySelectorAll('span, div, h1, h2, h3');
                    const suggested = [...allSpans].find(el => {
                        const text = el.textContent?.trim().toLowerCase() || '';
                        const before = getComputedStyle(el, '::before').content.replace(/['"]/g, '').toLowerCase();
                        const after = getComputedStyle(el, '::after').content.replace(/['"]/g, '').toLowerCase();
                        return text === 'suggested posts' || before === 'suggested posts' || after === 'suggested posts';
                    });
                    console.log(suggested);
                    if (!suggested) return;
                    console.log("Found Suggested: ", suggested);

                    const start = getTargetDiv(suggested);
                    console.log("Found Start: ", start);
                    if (!start) return;

                    let el = start.nextElementSibling; // use start.nextElementSibling if you want to keep the target div visible
                    console.log("EL AND START", el, start)
                    while (el) {
                        const next = el.nextElementSibling;
                        el.style.display = 'none';
                        el = next;
                    }
                }

                hideSuggestedAndBelow();

                // DMs only scroll disable
                function disableReelOverlayScroll() {
                    const reel = document.querySelector('[data-reel-type]');
                    if (!reel) return;

                    // walk up to find scrollable ancestor
                    let el = reel.parentElement;
                    while (el) {
                        const style = getComputedStyle(el);
                        if (style.overflowY === 'scroll' || style.overflowY === 'auto') {
                            el.style.overflowY = 'hidden';
                            break;
                        }
                        el = el.parentElement;
                    }
                }

                window.addEventListener("DOMContentLoaded", () => {
                    console.log("DOM ready");

                    const observer = new MutationObserver(() => {
                    hideSuggestedAndBelow();
                    const reel = document.querySelector('[data-reel-type]');
                    if (reel) {
                    disableReelOverlayScroll();
                    }
                    });
                    observer.observe(document.body, { childList: true, subtree: true });
                });

                // URL interceptor
                const _pushState = history.pushState.bind(history);
                const _replaceState = history.replaceState.bind(history);

                function interceptUrl(url) {
                    if (!url) return url;
                    return url.toString().replace(/\/reels\/([^/]+)\//, '/p/$1/');
                }

                history.pushState = function(state, title, url) {
                    return _pushState(state, title, interceptUrl(url));
                };

                history.replaceState = function(state, title, url) {
                    return _replaceState(state, title, interceptUrl(url));
                };
            "##,
                )
                .unwrap();
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
