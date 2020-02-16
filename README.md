# Wasm Custom ABI from Rust

This project showcases:
1. How to create a WebAssembly module from Rust using a custom ABI 
2. How to use this module from a host environment

## Requirements

Rust target `wasm32-unknown-unknown`.
You can install it with `rustup target add wasm32-unknown-unknown`.

## Instructions

First, let's compile our app into a WebAssembly Module.

```bash
rustc --target=wasm32-unknown-unknown app.rs
```

This should have produced a `app.wasm` file ready to be used anywhere.

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

This will output the following in shell:

```
Hello, World! from HOST_NAME
Do exit with code 0
Here are the logged calls: ["print", "exit"]
```
