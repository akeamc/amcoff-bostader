# A(mcof)F Bostäder

A custom [AF Bostäder](https://www.afbostader.se/) client.

## Developing

There is a Next.js project in `/frontend` that can be set up using [Bun](https://bun.sh/):

```sh
cd frontend
bun install # install dependencies
bun dev # start the server on :3000
```
The backend is located in `/api`; run it with Cargo as usual.

> [!IMPORTANT]
> Keep in mind that libpdfium is required for PDF rendering. Until [pdfium-render#151](https://github.com/ajrcarey/pdfium-render/issues/151) is resolved, the latest supported version of libpdfium is **128.0.6569.0**. Pre-built binaries are available at [bblanchon/pdfium-binaries](https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium%2F6569).
