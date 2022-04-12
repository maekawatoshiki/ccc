mod compile;

use compile::compile;
use lang_c::driver::{parse, Config};

fn main() {
    let config = Config::default();
    let result = parse(&config, "example.c").expect("Error occurred while parsing");
    compile(result).expect("Error occurred while compiling")
}
