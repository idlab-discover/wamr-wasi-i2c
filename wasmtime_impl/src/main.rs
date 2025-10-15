use wasmtime_impl::PingPongRunner;

// Main functie
fn main() {
    println!("run,type,time");
    PingPongRunner::new().unwrap().bench_guest();
}
