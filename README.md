# "Hello world!" wasm-pack library

🦀 Greeter demo using wasm-pack to create an npm package

<table align=center><td>

```js
import { greet } from "hello-world-wasm-pack-lib"

greet("Alan Turing")
//=> 'Hello Alan Turing!'
```

</table>

<p align=center>
  <a href="https://tsdocs.dev/docs/hello-world-wasm-pack-lib">Docs</a>
</p>

## Installation

```sh
npm install hello-world-wasm-pack-lib
```

## Usage

```js
import { greet } from "hello-world-wasm-pack-lib"

greet("Alan Turing")
//=> 'Hello Alan Turing!'
```

## Development

```sh
./just build
```

```sh
./just generate-patch
```
