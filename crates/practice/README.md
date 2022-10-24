# Practice wasm

## cargo install

- default
```bash
cargo install wasm-pack
```

- custom
```bash
cargo install wasm-pack --no-default-features
```

## compile
```bash
wasm-pack build --target nodejs
```

## run tests
```bash
wasm-pack test --node
```
