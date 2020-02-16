# Wasm Custom ABI (Rust)

This project showcases two things:
1. How to create a WebAssembly module from Rust using a custom ABI 
2. How to use this module from a host environment

## Requirements

Rust target `wasm32-unknown-unknown`.
You can install it with `rustup target add wasm32-unknown-unknown`.

## Instructions

First, let's compile the WebAssembly module.

```bash
rustc --target=wasm32-unknown-unknown app.rs
```

Now, you have a `app.wasm` file that you should be ready to use anywhere.
The module is defined as:

```wasm
(module
  (import "customabi" "print" (func $print (type $t0)))
  (import "customabi" "exit" (func $exit (type $t1)))
  ;; ...
)
```


Now we should be ready to run the `app.wasm` file our host environment!

```bash
cargo run
```

This should output this to your shell:

```
Hello, World! from HOST_NAME
Do exit with code 0
Here are the logged calls: ["print", "exit"]
```
