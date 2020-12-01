extern "C" {
    pub fn foo() -> i32;
    pub fn bar() -> f32;
}

fn main() {
    unsafe {
        println!("{}", foo());
        println!("{}", bar());
    }
}

#[no_mangle]
pub extern "C" fn test() -> i32 {
    123
}
