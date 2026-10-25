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
  <a href="https://github.com/readchek/extensions/issues">Issues</a>
</p>

Add the catalog to your Chek server. No account or token is required.

Repository URL: **https://extensions.chek.page/index.json**

## Sources

- **Pepper&Carrot**: English episodes from [David Revoy's official website](https://www.peppercarrot.com/en/webcomics/peppercarrot.html). Artwork is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); attribution and original credit pages are preserved. Chek is not affiliated with the author.

The source code license does not replace the publications' licenses. Extensions fetch publications from their original hosts; this repository does not bundle artwork.

## Development

Install Rust's `wasm32-wasip2` target and the [Chek CLI](https://github.com/readchek/chek), then run:

```sh
chek ext build --name 'Chek Extensions'
```

The generated `dist/index.json` and WASM components form a public extension repository.

## Deployment

Publish the catalog and its custom domain with Cloudflare CLI:

```sh
bun install --frozen-lockfile
bun run deploy
```

## License

Extension code is licensed under [MIT](LICENSE). Publications retain their source licenses.
