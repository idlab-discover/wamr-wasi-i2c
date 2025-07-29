use std::error::Error;

use host::{
    wamr_manager::{ run_guest_function, setup_module, setup_module_instance, setup_runtime },
};

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = setup_runtime()?;
    let module = setup_module(&runtime)?;
    let instance = setup_module_instance(&runtime, &module)?;
    let res = run_guest_function(&instance)?;
    println!("Host: Guest Output Value: {:?}", res);
    Ok(())
}
