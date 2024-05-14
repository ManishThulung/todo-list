// use std::env;
// use std::f32::consts::E;
// use todo::Config;
use std::process;
use std::io;

fn main() {

    // let config = Config::build(env::args()).unwrap_or_else(|err| {
    //     println!("args parsing error: {}", err);
    //     process::exit(1);
    // });

    // if let Err(e) = todo::add_todo(config) {
    //     println!("error while adding todo: {}", e);
    //     process::exit(1);
    // }

    loop {
        println!("enter 1 for adding todo");
        println!("enter 2 to see all todos");
        println!("enter 3 to exit");

        let mut action = String::new();

        io::stdin().read_line(&mut action).expect("failed to read a number");

        let action: i8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match action {
            1 => {
                println!("add task");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("failed to read a number");

                if let Err(e) = todo::add_todo(task) {
                    println!("error while adding todo: {}", e);
                    process::exit(1);
                }
            }
            2 => {
                println!("listed tasks");
                if let Err(e) = todo::read_todo() {
                    println!("error while adding todo: {}", e);
                    process::exit(1);
                }
                break;
            }
            3 => {
                println!("EXIT");
                break;
            }
            _ => {
                println!("WRONG COMMAND");
                continue;
            }
        }
    }
}
