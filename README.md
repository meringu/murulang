# Muru-lang

Reference implementation built on [pest](https://github.com/pest-parser/pest)!

Murulang compiles to WebAssembly and WASI

## Usage

1. Install rust and your favourite WebAssembly runtime (e.g. wasmtime).
1. Run the example with:

    ```sh
    cargo run -- build examples/example.muru
    wasmtime ./examples/example.wasm
    ```

Set the `-l=debug` flag during compilation to see all the various abstract syntax trees during compilation.
