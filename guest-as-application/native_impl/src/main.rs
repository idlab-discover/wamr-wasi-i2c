fn main() {
    let mut hw = native_impl::setup();
    native_impl::pingpong(&mut hw);
}
