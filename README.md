# ts_module

![Docker](resources/logo/docker-2496ed.svg)
![Node](resources/logo/node_16.svg)
![Typescript](resources/logo/ts.svg)
![Rust](resources/logo/rust.svg)
![Prettier](resources/logo/prettier.svg)
![Eslint](resources/logo/eslint.svg)

practice

* git submodule
* typescript lib
* web assembly with rust
* node addons with rust (napi)


## Project setup

### Install dependencies

```bash
npm install
```

### Build

- typescript

```bash
npm run build
```

- rust

```bash
npm run build:packages
```

### Run tests

- typescript

```bash
npm run test
```

```bash
npm run test:watch
```

```bash
npm run test:coverage
```

- wasm

```bash
npm run test:wasm
```

```bash
npm run test:wasm:watch
```

```bash
npm run test:wasm:coverage
```

- rust

```bash
cargo test
```

## Code style

### Eslint

```bash
npm run lint
```

### Prettier

```bash
npm run format:check
npm run format
```


