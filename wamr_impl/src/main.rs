use wamr_impl::PingPongRunner;

fn main() {
    println!("run,type,time");
    PingPongRunner::new().unwrap().bench_guest();
}
