# minify-html-ssr
For SSR WASM import.

## What it does

This library removes extraneous whitespaces between HTML tags. Single spaces are preserved where necessary.

## Directions

### Local
1. wasm-pack build --target=nodejs --out-dir path-to-node-project
2. import and add `const { minify_html } = require('../utils/minify-html/html_whitespace');` to your render pipeline.

```
res.send(minify_html(body));
```
### npm package
1. `npm install minify-html-ssr`
2. Minify the html body before sent down.

## Downsides
This is a runtime library. It is meant to work fast for you.  However, a build time solution that collapses whitespace might be better for your app.
