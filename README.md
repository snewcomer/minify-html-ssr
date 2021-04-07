# minify-html-ssr
For SSR WASM import.  Generates a /pkg wasm file to consume.  See: https://github.com/snewcomer/ember-fastboot-v2-example/tree/main/fastboot-server/middlewares

## Directions

### Local
1. wasm-pack build --target=nodejs --out-dir path-to-node-project
2. cp -r ../minify-html-ssr/pkg/* ./src/utils/minify-html-ssr
3. import and add `const { minify_html } = require('../utils/minify-html/html_whitespace');` to your render pipeline


### npm package
