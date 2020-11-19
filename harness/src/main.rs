extern "C" {
    pub fn foo() -> i32;
    pub fn bar() -> i32;
    pub fn baz() -> i32;
}

fn main() {
    unsafe {
        println!("{}", foo());
        println!("{}", bar());
        println!("{}", baz());
    }
}
