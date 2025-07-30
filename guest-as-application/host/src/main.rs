use std::error::Error;
use dhat;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use host::{
    wamr_manager::{ run_guest_function, setup_module, setup_module_instance, setup_runtime },
};

fn main() -> Result<(), Box<dyn Error>> {
    // let _profiler = dhat::Profiler::new_heap();
    let runtime = setup_runtime()?;
    let module = setup_module(&runtime)?;
    let instance = setup_module_instance(&runtime, &module)?;
    println!("Host: Address of the Instance: {:p}", &instance);
    let res = run_guest_function(&instance)?;
    println!("Host: Guest Output Value: {:?}", res);
    let module2 = setup_module(&runtime)?;
    let instance2 = setup_module_instance(&runtime, &module2)?;
    let res2 = run_guest_function(&instance2)?;
    println!("Host: Guest Output Value: {:?}", res2);
    Ok(())
}
