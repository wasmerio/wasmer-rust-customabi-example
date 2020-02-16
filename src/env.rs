// Here we deifne a custom environment that will be carried
// always next to our Wasm `Ctx`.
use wasmer_runtime::{Ctx, Memory};

#[derive(Debug)]
pub struct CustomAbiEnv {
    pub host_name: String,
    pub called_functions: Vec<String>,
}

impl CustomAbiEnv {
    pub fn new(host_name: String) -> Self {
        CustomAbiEnv {
            host_name,
            called_functions: vec![],
        }
    }

    pub fn log_call(&mut self, func_name: String) {
        self.called_functions.push(func_name);
    }

    pub fn get_memory_and_environment(
        ctx: &mut Ctx,
        mem_index: u32,
    ) -> (&Memory, &mut CustomAbiEnv) {
        unsafe { ctx.memory_and_data_mut(mem_index) }
    }
}
