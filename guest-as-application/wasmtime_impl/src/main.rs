use wasmtime_impl::{ run_pingpong, setup_runtime };

use anyhow::Result;

// Main functie
fn main() -> Result<()> {
    let (inst, mut store) = setup_runtime()?;
    run_pingpong(&inst, &mut store)?;
    Ok(())
}
