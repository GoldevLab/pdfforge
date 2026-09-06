# PDFForge

Merge, split, and compress PDFs. Convert pages to JPG. Extract selectable text. No account. Uploads up to 20 MB.

Pure Rust (`pdq` + `hayro`) on Resuma Flow. Compress is a document rewrite (unused objects dropped), not a kilobyte photo budget.

```bash
cd pdfforge
RESUMA_CSP=0 RESUMA_BODY_LIMIT=26214400 cargo run
# http://127.0.0.1:3000
```

Default theme is **Press** (ink/cobalt), locked — no theme picker.

| Path | Role |
|---|---|
| `/` | Hub |
| `/pdf-compress` `/pdf-merge` `/pdf-split` `/jpg-to-pdf` `/pdf-to-jpg` `/pdf-to-text` | SEO landings + tool |
| `/privacy` `/terms` `/extension` | Legal + Chrome extension |
| `/app` `/app/compress` … | Tool-only (no article) |

Google AdSense: set `ADSENSE_CLIENT` and `ADSENSE_SLOT` (or per-placement `ADSENSE_SLOT_*`). `/ads.txt` is served when the client is set. Chrome extension lives in `extension/` — zip that folder for the Web Store. After a custom domain: Fly `SITE_URL`, then edit `extension/config.js` + `manifest.json`. `CHROME_STORE_URL` fills the button on `/extension`.

APIs: `POST /api/compress`, `/api/merge`, `/api/split` (`range`, `mode=each`), `/api/to-jpg` (`range`, `dpi`), `/api/to-text` (`range`). Downloads `GET /d/{id}`.

Intended live URL: https://pdfforge.fly.dev — Fly org Gravitad, region `dfw`. Push to `main` deploys via GitHub Actions after the app exists on Fly.
