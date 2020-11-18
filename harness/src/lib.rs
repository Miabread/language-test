#[no_mangle]
pub extern "C" fn print_number(input: i32) {
    print!("{}", input);
}
