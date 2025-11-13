# Run JavaScript in WebAssembly

Checkout the [documentation](https://wasmedge.org/docs/category/develop-wasm-apps-in-javascript)

## Preparation
- Install Rust/Cargo
```bash
curl https://sh.rustup.rs -sSf | sh
```
- Install the `wasm32-wasi` target
```bash
rustup target add wasm32-wasi
```
- (for macOS) Install wasi-sdk for building wasi programs, [Install wasi-sdk](https://github.com/WebAssembly/wasi-sdk?tab=readme-ov-file#install)

## Build wasm
```bash
cargo build --target wasm32-wasi --release
```
for macOS, you may need to set the `WASI_SDK_PATH` environment variable before building:
```bash
export CC="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
cargo build --target wasm32-wasi --release
```
you will get the wasm file at `target/wasm32-wasip1/release/wasmedge_quickjs.wasm`

## Compile AOT wasm
- Install [WasmEdge](https://wasmedge.org/docs/install-wasmedge)
compile the wasm file to wasmEdge AOT format for better performance
```bash
wasmedgec target/wasm32-wasip1/release/wasmedge_quickjs.wasm wasmedge_quickjs_aot.wasm
```