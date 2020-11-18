extern "C" {
    pub fn foobar() -> i32;
}

fn main() {
    unsafe {
        println!("{}", foobar());
    }
}
