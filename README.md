# Chek Extensions

Public source extensions supported by Chek. The catalog requires no account or token.

Repository URL: **https://extensions.chek.page/index.json**

## Sources

- **Pepper&Carrot**: English episodes from [David Revoy's official website](https://www.peppercarrot.com/en/webcomics/peppercarrot.html). Artwork is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); attribution and original credit pages are preserved. Chek is not affiliated with the author.

The source code license does not replace the publications' licenses. Extensions fetch publications from their original hosts; this repository does not bundle artwork.

## Build

Install Rust's `wasm32-wasip2` target and the [Chek CLI](https://github.com/readchek/chek), then run:

```sh
chek ext build --name 'Chek Extensions'
```

The generated `dist/index.json` and WASM components form a public extension repository.

To publish the catalog and its custom domain with Cloudflare CLI:

```sh
bun install --frozen-lockfile
bun run deploy
```

Project Gutenberg awaits [file-based source support](https://github.com/readchek/chek/issues/74). Digital Comic Museum is not included while automated access remains unverified.
