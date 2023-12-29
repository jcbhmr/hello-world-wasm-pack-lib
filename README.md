# "Hello world!" wasm-pack library

🦀 Greeter demo using wasm-pack to create an npm package

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
