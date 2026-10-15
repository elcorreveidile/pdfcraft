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
cd apps/pdfcraft-web && trunk build --release   # output: dist/web
```

## Status of the rebrand

- [x] Web shell: page title
- [x] Links: no Discord button, no ArtCraft site link; Help menu points at WordNext and this fork
- [x] About dialog and start screen: no ArtCraft marks; upstream copyright line
- [x] Hosting: `vercel.json` + `packaging/vercel/build.sh` (static, Trunk build)
- [ ] Spanish (Spain, tuteo) as default language, language selector
- [ ] Catalogue pass: "PdfCraft" → "WordNext PDF" in the visible texts
- [ ] Icon: replace the upstream app icon with a WordNext one
- [ ] Landing page, install manifest and Vercel project / domain
