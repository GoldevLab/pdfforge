# PDFForge

Merge, split, and compress PDFs. Convert pages to JPG. Extract selectable text. No account. Uploads up to 20 MB.

Pure Rust (`pdq` + `hayro`) on Resuma Flow. Compress is a document rewrite (unused objects dropped), not a kilobyte photo budget.

```bash
cd pdfforge
RESUMA_CSP=0 RESUMA_BODY_LIMIT=26214400 cargo run
# http://127.0.0.1:3000
```

| Path | Role |
|---|---|
Default theme is **Press** (ink/cobalt). Theme picker also offers official Resuma palettes.

| Path | Role |
|---|---|
| `/` | Hub |
| `/pdf-compress` `/pdf-merge` `/pdf-split` `/jpg-to-pdf` `/pdf-to-jpg` `/pdf-to-text` | SEO landings + tool |
| `/privacy` `/terms` | Legal |
| `/app` `/app/compress` … | Tool-only (no article) |

APIs: `POST /api/compress`, `/api/merge`, `/api/split` (`range`, `mode=each`), `/api/to-jpg` (`range`, `dpi`), `/api/to-text` (`range`). Downloads `GET /d/{id}`.

Intended live URL: https://pdfforge.fly.dev — Fly org Gravitad, region `dfw`. Push to `main` deploys via GitHub Actions after the app exists on Fly.
