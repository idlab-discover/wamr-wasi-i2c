use wamr_impl::{ PingPongRunner };

fn main() {
    PingPongRunner::new().unwrap().pingpong();
}
