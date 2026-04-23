use std::env;
use std::process;
use accessh::Config;
use accessh::get_connected;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    get_connected(config);

}


