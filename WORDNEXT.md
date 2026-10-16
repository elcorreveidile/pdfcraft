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
- Icons are copied from `assets/app-icon` at build time (`pdfcraft-web.*`, the monogram on a document
  page, so it differs from WordNext Photo's icon). The desktop icons (`.ico`, `.icns`, hicolor) use the
  same artwork; `pdfcraft-web.svg` and the master `pdfcraft.svg` are the same file.
- The old root address `/?file=<url>` redirects to `/app/?file=<url>`; `/app` redirects to `/app/`.
- The generic release zip (`packaging/web/package.sh`) still holds the app alone, with relative URLs.

## Desktop installers and signing

`.github/workflows/instaladores.yml` ("Instaladores") builds the desktop installers on free GitHub
runners: macOS (universal `.dmg` + CLI), Windows (`x64`, `arm64`: `.msi` + portable zip) and Linux
(`x86_64`, `aarch64`: AppImage, deb, rpm, tar.gz).

- **Run it:** Actions → Instaladores → Run workflow (optionally a test version such as `0.8.1-rc.1`).
  The installers come out as workflow artifacts. Pushing a tag `v*` also publishes them as a GitHub
  Release.
- **Signing is automatic.** The packaging scripts sign whatever signing material they find and fall
  back to unsigned installers otherwise. Add the secrets below and the next run signs; nothing in the
  workflow changes. Each job's summary says whether that build was signed.
- Without secrets the installers work, but macOS Gatekeeper and Windows SmartScreen warn about an
  unknown developer.
- `release.yml` is the upstream pipeline (paid macOS runner, requires the secrets on the `release`
  branch, publishes under another organisation). It is left as it was and unused here.

Secrets (Settings → Secrets and variables → Actions → New repository secret):

| Platform | Secret | What it is |
| --- | --- | --- |
| macOS | `APPLE_CERTIFICATE` | "Developer ID Application" certificate exported as `.p12`, base64 (`base64 -i cert.p12 \| pbcopy`) |
| macOS | `APPLE_CERTIFICATE_PASSWORD` | Password chosen when exporting the `.p12` |
| macOS | `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` | Apple ID, an app-specific password and the team id: needed to notarize (without them the build is signed but not notarized) |
| macOS | `KEYCHAIN_PASSWORD` | Optional; random if unset |
| Windows | `WINDOWS_CERTIFICATE`, `WINDOWS_CERTIFICATE_PASSWORD` | Code-signing certificate `.pfx`, base64, and its password |
| Windows (alternative) | `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE` | Azure Trusted Signing, if there is no `.pfx` |

Linux needs no signing.

### Installer branding

The desktop installers use the WordNext PDF name and identifier:

- **Identifier:** `tech.wordnext.pdf` (macOS bundle id, Linux/Flatpak app id, Wayland app id, hicolor
  icon names). It replaces the upstream `ai.storyteller.pdfcraft`, so the OS treats this as a different
  app from the original: settings and file associations are not shared with it.
- **Visible names:** app name in the macOS `Info.plist`, MSI product, shortcuts, install folder and
  right-click menu entries on Windows, `.desktop` and AppStream name on Linux.
- **Publisher:** "WordNext" (MSI manufacturer, AppStream developer, package vendor), contact
  `info@wordnext.tech`, homepage `https://pdf.wordnext.tech`.

Kept as they were (internal names, changing them is a separate step):

- Executables and crates: `pdfcraft`, `pdfcraft-cli`, `pdfcraft-*`; installer file names
  (`pdfcraft-<version>-<platform>-<arch>.*`); the macOS bundle file `PdfCraft.app`, its executable and
  icon file names, and the DMG volume name.
- The settings folders (`%APPDATA%\PdfCraft`, `PdfCraftData`) and Windows registry key names.
- The upstream copyright line: the macOS plist says "WordNext and the PdfCraft authors".

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
- [x] Installer branding (identifier, visible names, publisher); file names and bundle file still PdfCraft
- [x] Desktop installers workflow (signs when the secrets exist, unsigned otherwise); not yet run
- [ ] Domain `pdf.wordnext.tech` on the Vercel project (the `.vercel.app` addresses sit behind Vercel login)
- [ ] `links.rs`: point `APP_PAGE` at the new domain once it is live
