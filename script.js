document.addEventListener('DOMContentLoaded', () => {
    const navButtons = {
        home: document.getElementById('nav-home'),
        publications: document.getElementById('nav-publications'),
        cv: document.getElementById('nav-cv')
    };

    const views = {
        home: document.getElementById('content-home'),
        publications: document.getElementById('content-publications'),
        cv: document.getElementById('content-cv')
    };

    function switchView(viewName) {
        // Hide all views
        Object.values(views).forEach(view => {
            view.style.display = 'none';
        });

        // Show the selected view
        views[viewName].style.display = 'flex';

        // Update navigation button styles
        Object.entries(navButtons).forEach(([name, button]) => {
            if (name === viewName) {
                button.classList.add('highlighted');
            } else {
                button.classList.remove('highlighted');
            }
        });
    }

    navButtons.home.addEventListener('click', () => switchView('home'));
    navButtons.publications.addEventListener('click', () => switchView('publications'));
    navButtons.cv.addEventListener('click', () => switchView('cv'));
});
