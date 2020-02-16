// This program will be compiled to WebAssembly using a Custom ABI
// We will then use Wasmer embedded to run this module.

// We define our own `start` function, so we tell Rust to not require us
// to have a main function.
#![no_main]
use std::str;

// Define the functions that this module will use from the outside world.
// In general, the set of this functions is what we define as an ABI.
// Here we define the "customabi" namespace for the imports,
// Otherwise it will be "env" by default
#[link(wasm_import_module = "customabi")]
extern "C" {
    fn print(ptr: *const u8, len: usize);
    fn exit(code: usize);
}

static HELLO: &'static str = "Hello, World!";

#[no_mangle]
pub fn start() {
    // Call the function we just imported and pass in
    // the offset of our string and its length as parameters.
    unsafe {
        print(HELLO.as_ptr(), HELLO.len());
        exit(0);
    }
}
