// Every page is a complete HTML document, and every internal link is a
// real <a href>. 
//
// There is exactly one copy of each page's content, and it lives in that page's own
// index.html. We fetch the real page and lift #content out of it
(function () {
    const content = document.getElementById('content');
    const navigation = document.getElementById('navigation');
    if (!content || !navigation) return;

    // pathname -> parsed page record
    const cache = new Map();

    // Guard against a slow fetch landing after a newer click already rendered
    let latest = 0;

    async function load(url) {
        const key = new URL(url, location.origin).pathname;
        if (cache.has(key)) return cache.get(key);

        const response = await fetch(url, { headers: { 'Accept': 'text/html' } });
        if (!response.ok) throw new Error('Failed to load ' + url);

        const doc = new DOMParser().parseFromString(await response.text(), 'text/html');
        const incoming = doc.getElementById('content');
        const nav = doc.getElementById('navigation');
        if (!incoming) throw new Error('No #content in ' + url);

        const record = {
            content: incoming.innerHTML,
            navigation: nav ? nav.innerHTML : null,
            title: doc.title,
            description: metaContent(doc, 'description'),
            canonical: canonicalHref(doc)
        };
        cache.set(key, record);
        return record;
    }

    function metaContent(doc, name) {
        const tag = doc.querySelector('meta[name="' + name + '"]');
        return tag ? tag.getAttribute('content') : null;
    }

    function canonicalHref(doc) {
        const tag = doc.querySelector('link[rel="canonical"]');
        return tag ? tag.getAttribute('href') : null;
    }

    function render(page) {
        content.innerHTML = page.content;
        if (page.navigation !== null) navigation.innerHTML = page.navigation;

        document.title = page.title;
        setHeadTag('meta[name="description"]', 'content', page.description);
        setHeadTag('link[rel="canonical"]', 'href', page.canonical);

        window.scrollTo(0, 0);
    }

    function setHeadTag(selector, attribute, value) {
        const tag = document.head.querySelector(selector);
        if (tag && value !== null) tag.setAttribute(attribute, value);
    }

    async function navigate(url, push) {
        const token = ++latest;

        let page;
        try {
            page = await load(url);
        } catch (e) {
            // 404 malformed response -- hand it to the browser
            location.href = url;
            return;
        }

        if (token !== latest) return; // superseded by a newer click
        if (push) history.pushState({}, '', url);
        render(page);
    }

    // Let the browser handle anything that isn't a plain left-click on a same-origin page
    function isPlainInternalClick(event, link) {
        if (event.defaultPrevented) return false;
        if (event.button !== 0) return false;
        if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return false;
        if (!link) return false;
        if (link.target && link.target !== '_self') return false;
        if (link.hasAttribute('download')) return false;
        if (link.origin !== location.origin) return false;
        if ((link.getAttribute('href') || '').startsWith('#')) return false;
        // Files (the CV PDF) are not pages to swap in
        if (/\.[a-z0-9]+$/i.test(link.pathname) && !/\.html?$/i.test(link.pathname)) return false;
        return true;
    }

    document.addEventListener('click', (event) => {
        const link = event.target.closest('a');
        if (!isPlainInternalClick(event, link)) return;

        event.preventDefault();
        if (link.href === location.href) return;
        navigate(link.href, true);
    });

    // Warm the cache
    navigation.addEventListener('mouseover', (event) => {
        const link = event.target.closest('a');
        if (link && link.origin === location.origin) load(link.href).catch(() => {});
    });

    window.addEventListener('popstate', () => {
        navigate(location.href, false);
    });
})();
