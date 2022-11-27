mod args;
mod build;
mod primitives;
mod print;
mod solve;

use std::{
    io::{stdin, stdout, Read, Write},
    process::exit,
};

use args::SudokuArgs;
use build::run as build;
use clap::Parser;
use colored::Colorize;
use inquire::Confirm;
use print::run as print;
use solve::run as solve;

fn main() {
    let cli = SudokuArgs::parse();

    let (input, empty_input) = if let Some(name) = cli.sudoku_input {
        (name, false)
    } else {
        (create_empty_sudoku_input(), true)
    };

    if empty_input && !cli.no_confirm {
        let ans = Confirm::new("No cell values provided.\nSolve an empty Sudoku?")
            .with_default(true)
            .prompt();

        match ans {
            Ok(true) => (),
            Ok(false) => exit_with_message("Exiting application"),
            Err(_) => println!("Error with questionnaire, try again later"),
        }
    }

    let initial_sudoku = build(input).expect("Building sudoku");

    println!("{}", "--------------- INPUT ---------------".yellow());

    print(&initial_sudoku);

    if !cli.no_confirm {
        press_btn_to_continue();
    }

    let solved_sudoku = solve(initial_sudoku).expect("Solving sudoku");

    println!("{}", "-------------- SOLVED ---------------".yellow());

    print(&solved_sudoku);
}

fn create_empty_sudoku_input() -> String {
    let mut result = String::from("");
    for _ in 0..81 {
        result.push('0');
    }
    return result;
}

fn exit_with_message(message: &str) {
    println!("{}", message);
    exit(0)
}

fn press_btn_to_continue() {
    let mut stdout = stdout();
    stdout
        .write(b"Press Enter to continue solving this sudoku...")
        .unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
