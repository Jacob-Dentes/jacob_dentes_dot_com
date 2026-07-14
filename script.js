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
            cache[url] = await response.text();
        }
        container.innerHTML = cache[url];
    }

    async function switchView(viewName) {
        await loadInto(content, tabs[viewName]);

        Object.entries(navButtons).forEach(([name, button]) => {
            if (name === viewName) {
                button.classList.add('highlighted');
            } else {
                button.classList.remove('highlighted');
            }
        });
    }

    // Event delegation: inject Hobbies markup
    content.addEventListener('click', (event) => {
        const worked = event.target.closest('[data-work]');
        if (!worked) return;

        const work = worked.dataset.work;
        if (work === '__back') {
            switchView('hobbies');
        } else {
            loadInto(content, 'hobbies/' + work + '.html');
        }
    });

    Object.keys(navButtons).forEach((name) => {
        navButtons[name].addEventListener('click', () => switchView(name));
    });

    switchView('home');
});
