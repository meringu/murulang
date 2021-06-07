# Muru-lang

Reference implementation built on [pest](https://github.com/pest-parser/pest)!

Murulang compiles to WebAssembly and WASI

## Usage

1. Install rust and wasmtime.
1. Run the example with:

    ```sh
    cargo run -- build example.muru
    wasmtime example
    echo $?
    ```

Set the `RUST_LOG=debug` environment variable during compilation to see all the various abstract syntax trees during compilation.
