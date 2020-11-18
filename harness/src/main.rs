extern "C" {
    fn foobar() -> i32;
}

fn main() {
    unsafe {
        println!("{}", foobar());
    }
}
