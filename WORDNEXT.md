# WordNext PDF (fork of PdfCraft)

WordNext PDF is WordNext's build of [PdfCraft](https://github.com/storytold/pdfcraft), an
open-source PDF application written in Rust (MIT OR Apache-2.0). This fork exists to ship a
browser build (WebAssembly) under the WordNext name, served from WordNext.

## Rules for this fork

- **Upstream copyright and licence stay.** `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` and
  `ATTRIBUTION.md` are not edited, except to add WordNext lines below the upstream ones.
- **ArtCraft marks never ship in our product.** `docs/brand/` is kept only because it is part of
  the upstream repository copy; the name, wordmark and "A" mark must not appear in the web build,
  our pages, installers or marketing. Product name: **WordNext PDF**
  (`pdfcraft_engine::links::PRODUCT_NAME`).
- **Keep the diff small.** Branding changes go in the web shell (`apps/pdfcraft-web`), in
  `crates/engine/src/links.rs` and in a few UI constants, so `git merge upstream/main` stays cheap.
  Crate and binary names stay `pdfcraft-*`.
- **Upstream remote:** `git remote add upstream https://github.com/storytold/pdfcraft.git`.

## Build the web version

```
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cd apps/pdfcraft-web && trunk build --release   # output: dist/web (the app alone)
```

## What Vercel serves

`packaging/vercel/build.sh` builds the app, then `packaging/vercel/assemble.sh` puts the site
together in `dist/site` (`vercel.json` publishes that folder):

| URL | What | Source |
|---|---|---|
| `/` and `/en/` | Landing page, Spanish and English (static HTML and CSS, no JavaScript) | `packaging/vercel/site/` |
| `/app/` | The app (Trunk output) with its install manifest and icons | `dist/web`, `packaging/vercel/site/app/` |
| `/robots.txt`, `/sitemap.xml` | Search engines | `packaging/vercel/site/` |

- `assemble.sh` needs neither Rust nor Trunk, so it can be run and checked with any `dist/web`.
- **Install manifest, no service worker.** The product and its build have no handwritten JavaScript
  (`CLAUDE.md`), so the app can be installed from the browser menu but does not work offline.
  Offline use would need a service worker written in Rust.
- Icons are copied from `assets/app-icon` at build time; there is one master set.
- The old root address `/?file=<url>` redirects to `/app/?file=<url>`; `/app` redirects to `/app/`.
- The generic release zip (`packaging/web/package.sh`) still holds the app alone, with relative URLs.

## Status of the rebrand

- [x] Web shell: page title
- [x] Links: no Discord button, no ArtCraft site link; Help menu points at WordNext and this fork
- [x] About dialog and start screen: no ArtCraft marks; upstream copyright line
- [x] Hosting: `vercel.json` + `packaging/vercel/build.sh` (static, Trunk build)
- [x] Spanish (Spain, tuteo) as default language, language selector
- [x] Catalogue pass: "PdfCraft" → "WordNext PDF" in the visible texts
- [x] Icon: the WordNext monogram replaces the upstream app icon
- [x] Landing page (es/en) and install manifest
- [x] Vercel project `wordnext-pdf`, deploying `main`
- [ ] Domain `pdf.wordnext.tech` on the Vercel project (the `.vercel.app` addresses sit behind Vercel login)
- [ ] `links.rs`: point `APP_PAGE` at the new domain once it is live
