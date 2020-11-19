extern "C" {
    pub fn foobar() -> i32;
}

fn main() {
    unsafe {
        println!("{}", foobar());
    }
}

#[no_mangle]
pub extern "C" fn test() -> i32 {
    123
}
