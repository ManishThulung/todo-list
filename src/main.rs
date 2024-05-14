use std::env;
use std::process;
use todo::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("args parsing error: {}", err);
        process::exit(1);
    });

    if let Err(e) = todo::add_todo(config) {
        println!("error while adding todo: {}", e);
        process::exit(1);
    }
}
