# Muru-lang

This reference implementation compiles to WebAssembly and WASI.

## Usage

1. Install rust.
1. Build `muru`:

    ```sh
    cargo build --release
    export PATH=$PATH:$(pwd)/target/release
    ```

1. Run the example:

    ```sh
    muru run ./examples/example.muru
    ```

1. Or build the example and use your favourite WASI runtime (e.g. wasmtime):

    ```sh
    muru build ./examples/example.muru
    wasmtime ./examples/example.wasm
    ```

Set the `-l=debug` flag to see all the various abstract syntax trees during compilation.
