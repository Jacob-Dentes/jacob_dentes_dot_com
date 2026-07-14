document.addEventListener('DOMContentLoaded', () => {
    const content = document.getElementById('content');

    const navButtons = {
        home: document.getElementById('nav-home'),
        publications: document.getElementById('nav-publications'),
        cv: document.getElementById('nav-cv'),
        hobbies: document.getElementById('nav-hobbies')
    };

    const tabs = {
        home: 'tabs/home.html',
        publications: 'tabs/publications.html',
        cv: 'tabs/cv.html',
        hobbies: 'tabs/hobbies.html'
    };

    // Cache fetched partials by URL so re-visiting a tab or work is instant.
    const cache = {};

    async function loadInto(container, url) {
        if (!(url in cache)) {
            const response = await fetch(url);
            if (!response.ok) throw new Error('Failed to load ' + url);
            cache[url] = await response.text();
        }
        container.innerHTML = cache[url];
    }

    function highlight(viewName) {
        Object.entries(navButtons).forEach(([name, button]) => {
            button.classList.toggle('highlighted', name === viewName);
        });
    }

    async function showTab(viewName) {
        await loadInto(content, tabs[viewName]);
        highlight(viewName);
    }

    async function showWork(work) {
        // A work lives under the Hobbies tab, so light up that nav button.
        highlight('hobbies');
        try {
            await loadInto(content, 'hobbies/' + work + '.html');
        } catch (e) {
            // fall back to home
            await showTab('home');
        }
    }

    // Resolve a location string (a tab name or a work slug) to a view.
    async function route(loc) {
        if (loc && loc in tabs) {
            await showTab(loc);
        } else if (loc) {
            await showWork(loc);
        } else {
            await showTab('home');
        }
    }

    // Reflect the current location in the URL so it can be bookmarked/shared.
    function pushLoc(loc) {
        const url = new URL(window.location);
        if (loc && loc !== 'home') {
            url.searchParams.set('loc', loc);
        } else {
            url.searchParams.delete('loc');
        }
        history.pushState({ loc: loc }, '', url);
    }

    function navigate(loc) {
        pushLoc(loc);
        route(loc);
    }

    Object.keys(navButtons).forEach((name) => {
        navButtons[name].addEventListener('click', () => navigate(name));
    });

    // Event delegation: clicking a work (or a "back" control) inside #content.
    content.addEventListener('click', (event) => {
        const worked = event.target.closest('[data-work]');
        if (!worked) return;

        const work = worked.dataset.work;
        navigate(work === '__back' ? 'hobbies' : work);
    });

    // Browser back/forward buttons.
    window.addEventListener('popstate', (event) => {
        const loc = (event.state && event.state.loc) ||
            new URL(window.location).searchParams.get('loc');
        route(loc);
    });

    // Initial load: honor ?loc= if present, otherwise show Home.
    route(new URL(window.location).searchParams.get('loc'));
});
