fn main() {
    cc::Build::new().object("../indev/output.o").compile("foo");
}
