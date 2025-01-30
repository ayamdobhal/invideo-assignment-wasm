# invideo-assignment-wasm

WebAssembly bindings for an arithmetic expression evaluator to be used with the Frontend for the assignment.

## Requirements

- Rust
- wasm-pack

## Compiling

Ensure that Rust is installed. If not it can be installed using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Next, install `wasm-pack`:

```bash
cargo install wasm-pack
```

To compile the binary, run:

```bash
wasm-pack build --target web
```

The binaries can be found in the `pkg` directory.

## Testing

Tests can be run using:

```bash
wasm-pack test --firefox
```

Where `firefox` can be replaced with `chrome` or `safari`.
