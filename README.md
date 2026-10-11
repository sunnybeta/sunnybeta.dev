# Sunny Béta — portfolio

Two sites, one Rust server. The pages are rendered from Askama templates at
request time — nothing is written to disk. Everything the pages render lives in
`src/content.rs` as data — edit that file, not the templates.

## Routes

Retro is the default site at `/`; the professional site starts at `/pro`.

| Route | Page | Stylesheet |
| --- | --- | --- |
| `/`, `/retro` | Indie-web single page (everything on one page) | `/static/css/retro.css` |
| `/pro` | Professional home | `/static/css/style.css` |
| `/experience` | Experience | `/static/css/style.css` |
| `/projects` | Projects | `/static/css/style.css` |
| `/pricing` | Pricing | `/static/css/style.css` |
| `/testimonials` | Testimonials | `/static/css/style.css` |
| `/blogs` | Blogs | `/static/css/style.css` |
| `/contact` | Email + PGP key | `/static/css/style.css` |

## Commands

```sh
make         # cargo build
make serve   # cargo run — serve on http://localhost:2382
make clean   # cargo clean
```

Set `PORT` to listen somewhere other than 2382 (`PORT=3000 cargo run`).

## Serving

The crate is a library plus a small binary: `src/lib.rs` exposes the router and
`src/main.rs` runs the server. Page handlers live in `src/routes.rs`, one
Askama template per page. The hand-written assets all live under `static/` and
are served straight off the repo — they are never copied anywhere:

- `static/css/style.css` and `static/css/retro.css` — the stylesheets.
- `static/img/favicon.svg` — the favicon.
- `static/files/pgp-key.asc` — the public PGP key.
- `static/js/` — the browser scripts (`retro.js`, `copy.js`).

The whole folder is mounted at `/static`, so those files are served at
`/static/css/…`, `/static/img/…`, `/static/files/…` and `/static/js/…`. The
server looks for `static/` beside the current working directory and falls back
to the crate directory, so it can be launched from anywhere — set `STATIC_DIR`
to point at the folder explicitly.

Links are extensionless (`/pricing`); the router maps them to their handlers.

## Editing content

- `person` — name, email, location, socials
- `facts`, `services`, `skills` — home page blocks
- `tiers`, `add_ons` — pricing packages
- `projects`, `testimonials`, `posts`, `roles` — page content
- `retro` — status lines, badges, webring, guestbook note, plus the random
  pools behind the navbar meta strip, word art, captions, awards and stickers:
  `last_updated`, `quest`, `built_with`, `fortune`, `word_art`, `captions`,
  `awards`, `stickers`, `ticker`.

The page types live in `src/templates.rs`; the markup is split across
`templates/layouts/` and `templates/partials/` (`head`, `header`, `footer`,
`retro_head`, `retro_header`, `retro_footer`, `retro_art`). Nothing is
hardcoded in the HTML.

The pools are serialised into `<div id="retro-data" hidden>` as `|`-separated
`data-*` attributes, and `static/js/retro.js` picks from them in the browser.
`data-random="<pool>"` on any element fills it from that pool; a new pool needs
a matching attribute on `#retro-data` and (if it should be re-rollable) an entry
in `rerollAll()`.

## The retro art library

`templates/partials/retro_art.html` holds an inline `<svg>` sprite — 25
`<symbol>`s: 10 `88x31` buttons (`class="sym-btn"`) and 15 `32x32` pixel tiles
(`class="sym-tile"`). Because the sprite is inline in the page, it inherits the
page's CSS, so the buttons use the pixel font and the tiles animate.

- Reference a symbol with `<svg class="art"><use href="#tile-cat"/></svg>`
  (`.art` = 32px, `.art-lg` = 64px, `.art-btn` = 88x31).
- The JS reads the catalogue from the sprite itself — `data-label` on each
  symbol supplies its caption — so adding a symbol automatically adds it to the
  random image widget, the `BUTTON.WALL` and the `DECOR.ZIP` scrapbook. No
  second list to update.
- Every button in the wall that is labelled `RANDOM SITE` re-rolls the page.

All randomness is picked on page load and on an explicit click (`[ re-roll all ]`,
`[ roll again ]`, `[ next image ]`) — no timers, and animations are switched off
under `prefers-reduced-motion`.

## To do

- PGP fingerprint is a stub (`XXXX XXXX ...`) in `templates/layouts/retro.html`
  and `templates/layouts/pro_contact.html`.
- Blog posts are list entries only — no article pages yet.
- The guestbook form is deliberately non-functional; its caption says so.
