# jacobdentes.com

Personal site. Static, no build step — deployed straight to GitHub Pages.

Tabs are HTML partials in `tabs/` loaded at runtime by `script.js` and injected into
`index.html`. Creative works live in `hobbies/`; each is its own HTML fragment listed in
`tabs/hobbies.html` (internal works use a `data-work` button; external works are plain
`target="_blank"` links).

## Local preview

Because tabs/works are loaded via `fetch()`, open the site through a local web server,
not by double-clicking `index.html` (`file://` blocks `fetch`):

```
python -m http.server 8000
```

then visit http://localhost:8000/.
