use std::io;
use std::io::prelude::*;

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run_app() -> Result<(), ()> {
    loop {
        print_prompt();
        let input = read_input();

        if input == ".exit" {
            return Ok(());
        } else {
            println!("Unrecognized command '{}'.", input)
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap()
}
