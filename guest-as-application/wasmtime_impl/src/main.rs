use wasmtime_impl::PingPongRunner;

// Main functie
fn main() {
    let mut runner = PingPongRunner::new().unwrap();
    runner.pingpong();
}
