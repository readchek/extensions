<p align="center">
  <img src="https://raw.githubusercontent.com/readchek/chek.page/main/public/assets/logos/mark.svg" alt="Chek" width="80" height="80" />
</p>

<h1 align="center">Chek Extensions</h1>

<p align="center">Official public source extensions for <a href="https://github.com/readchek/chek">Chek</a>.</p>

<p align="center">
  <a href="https://extensions.chek.page/index.json">Catalog</a>
  ·
  <a href="https://github.com/readchek/chek/blob/main/docs/extension-repositories.md">Documentation</a>
  ·
  <a href="https://github.com/readchek/chek/issues/new/choose">Report a problem</a>
</p>

Add the catalog to your Chek server. No account or token is required.

Repository URL: **https://extensions.chek.page/index.json**

## Sources

| Source | What it offers |
| --- | --- |
| **Pepper&Carrot** | English episodes from [David Revoy's official website](https://www.peppercarrot.com/en/webcomics/peppercarrot.html), under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/), with attribution and the original credit pages kept |
| **Project Gutenberg Classics** | Twelve curated English classics, such as Pride and Prejudice, Alice's Adventures in Wonderland and The Adventures of Sherlock Holmes, opened as EPUBs with their source, license notice and credits |

- Chek isn't affiliated with the authors or hosts.
- Extensions fetch publications from their original hosts. This repository
  bundles no artwork, and its code license doesn't replace the publications'
  licenses.
- Every Gutenberg edition must be marked public domain in the USA; copyright
  rules elsewhere may differ. Gutenberg doesn't supply content ratings.

## Development

Install Rust's `wasm32-wasip2` target and the [Chek CLI](https://github.com/readchek/chek), then run:

```sh
chek ext build --name 'Chek Extensions'
```

The generated `dist/index.json` and WASM components form a public extension repository.

Notes on Gutenberg:

- It needs Chek server 0.2.0 or later, which can serve books and files
  alongside comic pages. Older servers can't read these books.
- Metadata comes from Gutenberg's [documented RDF feeds](https://www.gutenberg.org/ebooks/offline_catalogs.html), cached for one day.
- Search runs over the curated list without network requests.

## Deployment

Every push to `main` builds the catalog and publishes it at
extensions.chek.page through the Deploy workflow, which signs in with the
`CLOUDFLARE_API_TOKEN` repository secret. To redeploy without a push, run the
workflow from the Actions tab.

To publish from your own machine, sign in with `cf auth login` and run:

```sh
bun install --frozen-lockfile
bun run deploy
```

## License

Extension code is licensed under [MIT](LICENSE). Publications retain their source licenses.
