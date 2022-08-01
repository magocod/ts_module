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


## Project setup

### Compile rust code to javascript (packages)

Move to packages and compile rust projects
```bash
cd packages/practice
```
run
```bash
wasm-pack build --target nodejs
```

### Install dependencies

```bash
npm install
```

### Translate code to javascript (dist)

```bash
npm run build
```

### Run tests

```bash
npm run test
npm run test:watch

npm run test:coverage
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


