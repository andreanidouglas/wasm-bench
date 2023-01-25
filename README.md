# benchmark tool to test http responses from Fermyon (and possible other wasm targets)

## Deploy the wasm project

```bash
cd wasm-bench/
cargo build --target wasm32-wasi --release
spin deploy
```

once deployed, get the url of your deployment

```bash
export WASM_ENDPOINT=https://yourwasmendpoing.com
cd ./test-bench
cargo run --release
```





