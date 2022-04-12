mod compile;

use compile::compile;
use lang_c::driver::{parse, Config};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ccc")]
pub struct Opt {
    pub input_file: String,

    #[structopt(short = "o")]
    pub output_file: Option<String>,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let config = Config::default();
    let result = parse(&config, opt.input_file).expect("Error occurred while parsing");

    compile(&result).expect("Error occurred while compiling")
}
