# minify-html-ssr
For SSR WASM import.  Generates a /pkg wasm file to consume.  See: https://github.com/snewcomer/ember-fastboot-v2-example/tree/main/fastboot-server/middlewares

## Directions

### Local
1. wasm-pack build --target=nodejs --out-dir path-to-node-project
2. cp -r ../minify-html-ssr/pkg/* ./src/utils/minify-html-ssr
3. import and add `const { minify_html } = require('../utils/minify-html/html_whitespace');` to your render pipeline


### npm package
1. npm install `minify-html-ssr`
2. Minify the html body before sent down. [example](https://github.com/snewcomer/ember-alt-fastboot-app-server/blob/289cd96d88867ec997da44e19eec621a65ea418b/src/middlewares/fastboot.js#L124)
