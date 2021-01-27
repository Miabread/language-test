fn main() {
    println!("cargo:rerun-if-changed=../indev/output.o");
    cc::Build::new().object("../indev/output.o").compile("foo");
}
