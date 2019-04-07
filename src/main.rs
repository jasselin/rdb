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
        if input.is_empty() {
            continue;
        }

        let first_char = input.chars().next().unwrap();
        if first_char == '.' {
            match do_meta_command(&input) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input);
                    continue;
                }
                MetaCommandResult::Exit => {
                    println!("Goodbye.");
                    return Ok(());
                }
            }
        }

        let (result, statement) = prepare_statement(&input);
        match result {
            PrepareResult::Success => (),
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}", input);
                continue;
            }
        };

        execute_statement(&(statement.unwrap()));
        println!("Executed.");
    }
}

fn print_prompt() {
    print!("rdb > ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap()
}

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
    Exit,
}

fn do_meta_command(input: &str) -> MetaCommandResult {
    match input {
        ".exit" => MetaCommandResult::Exit,
        _ => MetaCommandResult::UnrecognizedCommand,
    }
}

enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    _type: StatementType,
}

fn prepare_statement(input: &str) -> (PrepareResult, Option<Statement>) {
    if input.starts_with("insert") {
        let statement = Statement {
            _type: StatementType::Insert,
        };
        return (PrepareResult::Success, Some(statement));
    }
    if input.starts_with("select") {
        let statement = Statement {
            _type: StatementType::Select,
        };
        return (PrepareResult::Success, Some(statement));
    }
    (PrepareResult::UnrecognizedStatement, None)
}

fn execute_statement(statement: &Statement) {
    match statement._type {
        StatementType::Insert => println!("This is where we would do an insert."),
        StatementType::Select => println!("This is where we would do a select."),
    }
}
