# Sunny Béta — portfolio

Two sites, one generator. Everything the pages render lives in `main.go` as
variables — edit that file, not the templates.

## Routes

Every page is generated into `public/`, which is the deploy root — upload that
folder on its own. Retro is the default site at `/`; the professional site
starts at `/pro`.

| Route | File | Page | Stylesheet |
| --- | --- | --- | --- |
| `/`, `/retro` | `public/index.html` | Indie-web single page (everything on one page) | `retro.css` |
| `/pro` | `public/pro.html` | Professional home | `style.css` |
| `/experience` | `public/experience.html` | Experience | `style.css` |
| `/projects` | `public/projects.html` | Projects | `style.css` |
| `/pricing` | `public/pricing.html` | Pricing | `style.css` |
| `/testimonials` | `public/testimonials.html` | Testimonials | `style.css` |
| `/blogs` | `public/blogs.html` | Blogs | `style.css` |
| `/contact` | `public/contact.html` | Email + PGP key | `style.css` |

## Commands

```sh
make        # generate public/ (HTML + copied assets)
make serve  # generate, then serve public/ on http://localhost:8080
make clean  # delete public/
```

## Build output

`public/` is entirely generated — never edit files inside it, `make clean`
deletes the whole folder. `make` writes the HTML and copies in the files that
belong at the web root but are hand-written:

- `style.css` and `retro.css` stay at the repo root as the source of truth and
  are copied into `public/` on every build, so keep editing them there.
- `pgp-key.asc` and the whole `static/` folder (JS and favicon) are copied in
  the same way.

The result is a self-contained folder: `public/index.html`, `public/pro.html`,
`public/pricing.html`, `public/testimonials.html`, `public/blogs.html`,
`public/projects.html`, `public/experience.html`, `public/contact.html`,
`public/style.css`, `public/retro.css`, `public/static/` and
`public/pgp-key.asc`. Links are extensionless (`/pricing`), so a host needs to
serve `pricing.html` for `/pricing` — GitHub Pages and Netlify do this out of
the box.

## Editing content

- `person` — name, email, location, socials
- `facts`, `services`, `skills` — home page blocks
- `tiers`, `addOns` — pricing packages
- `projects`, `testimonials`, `posts`, `roles` — page content
- `retro` — status lines, badges, webring, guestbook note, plus the random
  pools behind the navbar meta strip, word art, captions, awards and stickers:
  `LastUpdated`, `Quest`, `BuiltWith`, `Fortune`, `WordArt`, `Captions`,
  `Awards`, `Stickers`, `Ticker`.

Templates live in `templates/` and are split into partials
(`head`, `header`, `footer`, `retro_head`, `retro_header`, `retro_footer`,
`retro_art`). Nothing is hardcoded in the HTML.

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
- The guestbook form is deliberately non-functional (the site is static); its
  caption says so.