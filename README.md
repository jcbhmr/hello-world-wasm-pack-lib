# "Hello world!" wasm-pack library

🦀 Greeter demo using wasm-pack to create an npm package

<table align=center><td>

```js
import { greet, Calculator } from "hello-world-wasm-pack-lib";

greet("Alan Turing");
//=> 'Hello Alan Turing!'

const calc = new Calculator();
calc.add(10);
console.log(calc.value);
//=> 10
calc.mult(20);
console.log(calc.value);
//=> 200
```

</table>

<p align=center>
  <a href="https://tsdocs.dev/docs/hello-world-wasm-pack-lib">Docs</a>
  | <a href="https://www.npmjs.com/package/hello-world-wasm-pack-lib">npm package</a>
</p>

🦀 Written in Rust! \
🟪 Compiled to WebAssembly \
🟨 Exposed to JavaScript \
🎉 Great way to see what wasm-pack can do \
👩‍⚖️ [0BSD licensed](./LICENSE) template

## Installation

![npm](https://img.shields.io/static/v1?style=for-the-badge&message=npm&color=CB3837&logo=npm&logoColor=FFFFFF&label=)
![Yarn](https://img.shields.io/static/v1?style=for-the-badge&message=Yarn&color=2C8EBB&logo=Yarn&logoColor=FFFFFF&label=)
![pnpm](https://img.shields.io/static/v1?style=for-the-badge&message=pnpm&color=222222&logo=pnpm&logoColor=F69220&label=)
![Deno](https://img.shields.io/static/v1?style=for-the-badge&message=Deno&color=000000&logo=Deno&logoColor=FFFFFF&label=)
![Bun](https://img.shields.io/static/v1?style=for-the-badge&message=Bun&color=000000&logo=Bun&logoColor=FFFFFF&label=)
![jsDelivr](https://img.shields.io/static/v1?style=for-the-badge&message=jsDelivr&color=E84D3D&logo=jsDelivr&logoColor=FFFFFF&label=)

You can install this package using your favorite npm package manager like npm,
[Yarn], [pnpm], or [Bun].

```sh
npm install hello-world-wasm-pack-lib
```

If you're using [Deno] you can import it straight from npm:

```js
import {} from "npm:hello-world-wasm-pack-lib";
```

If you prefer to go buildless in the browser you can use an npm CDN like
[jsDelivr] or [esm.sh].

```html
<script type="module">
  import {} from "https://esm.run/hello-world-wasm-pack-lib";
</script>
```

## Usage

![Node.js](https://img.shields.io/static/v1?style=for-the-badge&message=Node.js&color=339933&logo=Node.js&logoColor=FFFFFF&label=)
![Deno](https://img.shields.io/static/v1?style=for-the-badge&message=Deno&color=000000&logo=Deno&logoColor=FFFFFF&label=)
![Bun](https://img.shields.io/static/v1?style=for-the-badge&message=Bun&color=000000&logo=Bun&logoColor=FFFFFF&label=)
![Browser](https://img.shields.io/static/v1?style=for-the-badge&message=Browser&color=4285F4&logo=Google+Chrome&logoColor=FFFFFF&label=)

You should be able to just import and use this JavaScript package like any
other. The WASM magic is hidden behind a really nice wrapper layer.

```js
import { greet, Calculator, get_report } from "hello-world-wasm-pack-lib";

greet("Alan Turing");
//=> 'Hello Alan Turing!'

const report = get_report();
console.log(report);
//=> Report {
//   dog_count: 5,
//   bouncy_castles: 3,
//   fun_percent: 76.8
// }
```

[📚 You can find complete API documentation on the docs website](https://tsdocs.dev/docs/hello-world-wasm-pack-lib)

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)
![JavaScript](https://img.shields.io/static/v1?style=for-the-badge&message=JavaScript&color=222222&logo=JavaScript&logoColor=F7DF1E&label=)

To build the JavaScript package just run `./just build`. Then you can `cd` into
the `pkg/` folder if you want and launch a REPL to try it out!

```sh
./just build
cd pkg
node
```

If you need to edit the `pkg.patch` file, you can do so via the below command.
It will create two folders: `a/` and `b/`. Edit anything in the `b/` folder and
a `.patch` will be generated against the `a/` folder and saved to `pkg.patch`.

```sh
./just generate-patch
# Pauses while you make changes...
```

<!-- prettier-ignore-start -->
[Yarn]: https://yarnpkg.com/
[pnpm]: https://pnpm.io/
[Bun]: https://bun.sh/
[Deno]: https://deno.com/
[jsDelivr]: https://www.jsdelivr.com/
[esm.sh]: https://esm.sh/
<!-- prettier-ignore-end -->
