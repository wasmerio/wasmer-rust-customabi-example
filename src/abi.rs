use wasmer_runtime::{Array, Ctx, WasmPtr};
use crate::env::CustomAbiEnv;

// Let's define our "print" function.
pub fn print(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {
    let (memory, env) = CustomAbiEnv::get_memory_and_environment(ctx, 0);

    // Use helper method on `WasmPtr` to read a utf8 string
    let string = ptr.get_utf8_string(memory, len).unwrap();

    // Print it!
    println!("{} from {}", string, env.host_name);

    // We also log the function call to the environment
    env.log_call("print".to_string());
}


// Let's define our "exit" function.
pub fn exit(ctx: &mut Ctx, code: u32) {
    let (_, env) = CustomAbiEnv::get_memory_and_environment(ctx, 0);

    println!("Do exit with code {}", code);

    // Log this call to the environment
    env.log_call("exit".to_string());
}
