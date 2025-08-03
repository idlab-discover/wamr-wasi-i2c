use wamr_impl::{ setup_runtime, run_pingpong };

fn main() {
    let (_runtime, _module, instance) = setup_runtime().expect("Oeps");
    run_pingpong(&instance).expect("Oeps");
}
