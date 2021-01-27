extern "C" {
    fn sonance_main() -> u8;
}

fn main() {
    unsafe {
        print_int(sonance_main());
    }
}

pub extern "C" fn print_int(int: u8) -> u8 {
    println!("{}", int);
    int
}

pub extern "C" fn print_char(int: u8) -> u8 {
    println!("{}", int as char);
    int
}

pub extern "C" fn add_int(left: u8, right: u8) -> u8 {
    left + right
}
