use wamr_impl::{ setup_runtime, run_pingpong };

fn main() {
    let (_runtime, _module, instance, func) = setup_runtime().expect("[WAMR] Runtime Setup Failed");
    run_pingpong(&instance, &func).expect("[WAMR] Running PingPong Failed");
}
