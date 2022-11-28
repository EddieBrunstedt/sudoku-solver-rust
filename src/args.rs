use clap::{ArgGroup, Parser};

const AFTER: &'static str = "FORMAT OF INPUT:
----------------
Values 0-9 where 0 represents an empty cell.
Any other character will be stripped away.

Examples:
530070000-600195000-098000060-800060003-400803001-700020006-060000280-000419005-000080079
002810700/400200000/080009060/000600020/005040300/100007008/090000050/006000400/300000001

Tip:
If you use values from file (with '-f' flag), feel free to use line breaks
in the file as they will be stripped away automatically.";

#[derive(Parser)]
#[command(author, version, about = "Solve any Sudoku using a backtracking algorithm.", long_about = None, after_help = AFTER)]
#[command(group(ArgGroup::new("input")
                .required(true)
                .args(["input_from_stdin", "input_from_file"]))
)]
pub struct SudokuArgs {
    /// Each cell's value from left to right
    #[arg(short = 's', long, value_name = "values")]
    pub input_from_stdin: Option<String>,

    /// Read values from a file
    #[arg(short = 'f', long, value_name = "file_path")]
    pub input_from_file: Option<String>,
}
