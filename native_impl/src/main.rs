fn main() {
    let mut hw = native_impl::setup();

    println!("run,type,time");
    for _ in 0..1000 {
        native_impl::pingpong(&mut hw);
    }
}
