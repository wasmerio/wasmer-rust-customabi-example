extern crate wasmer_runtime;

mod abi;
mod env;

use wasmer_runtime::{error, func, imports, instantiate};
use std::ffi::c_void;

use crate::env::CustomAbiEnv;

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] =
    include_bytes!("../app.wasm");

fn main() -> error::Result<()> {
    // Here we define an environment that will be attached to the
    // Wasm context
    let env_generator = move || {
        fn destructor(data: *mut c_void) {
            unsafe { drop(Box::from_raw(data as *mut CustomAbiEnv)); }
        }
        let custom_abi_env = Box::new(CustomAbiEnv::new("HOST_NAME".to_string()));
        (
            Box::into_raw(custom_abi_env) as *mut c_void,
            destructor as fn(*mut c_void),
        )
    };
    
    let import_object = imports! {
        env_generator,
        // Define the "customabi" namespace that was implicitly used
        // by our sample application.
        "customabi" => {
            // the func! macro autodetects the signature
            "print" => func!(abi::print),
            "exit" => func!(abi::exit),
        },
    };

    // Compile our webassembly into an `Instance`.
    let mut instance = instantiate(WASM, &import_object)?;

    // Call our start function!
    instance.call("start", &[])?;

    let (_, env) = CustomAbiEnv::get_memory_and_environment(instance.context_mut(), 0);
    
    println!("Here are the logged calls: {:?}", env.called_functions);
    Ok(())
}
