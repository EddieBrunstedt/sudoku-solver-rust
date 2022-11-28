mod args;
mod build;
mod primitives;
mod print;
mod solve;

use args::SudokuArgs;
use build::run as build;
use clap::Parser;
use colored::Colorize;
use print::run as print;
use solve::run as solve;
use std::{
    error::Error,
    fs,
    io::{stdin, stdout, Read, Write},
    process::exit,
};

fn main() {
    let cli = SudokuArgs::parse();
    let mut input: String = String::new();

    if let Some(file_path) = cli.input_from_file {
        println!("Looking for Sudoku values in file: {}", file_path);
        let input_from_file = get_string_from_file(&file_path);

        if let Err(err) = input_from_file {
            exit_with_message(&*err.to_string());
        } else {
            input = input_from_file.unwrap();
        }
    };

    if let Some(stdin_string) = cli.input_from_stdin {
        input = stdin_string;
    }

    let initial_sudoku = build(input).expect("Building sudoku");

    println!();
    println!("{}", "You have provided these values:".yellow());

    print(&initial_sudoku);

    press_btn_to_continue();

    let solved_sudoku = solve(initial_sudoku).expect("Solving sudoku");

    println!();
    println!("{}", "This is the solved Sudoku".yellow());

    print(&solved_sudoku);
}

fn get_string_from_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    return Ok(file_content);
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
