mod dhat_tests;

use dhat;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    for test in &dhat_tests::TESTS {
        test();
    }
}
