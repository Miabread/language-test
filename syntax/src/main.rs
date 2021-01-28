use syntax::parse;

fn main() {
    let input = std::fs::read("indev/input.son").expect("couldn't read source file");
    let _ = dbg!(parse(&input));
}
