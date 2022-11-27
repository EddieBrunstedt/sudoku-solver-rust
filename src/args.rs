use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SudokuArgs {
    /// Each cell's value from left to right of the sudoku where 0 represents an empty cell
    #[arg(short, long, value_name = "CELL_VALUES")]
    pub sudoku_input: Option<String>,

    /// Solving your sudoku, no questions asked
    #[arg(short, long)]
    pub no_confirm: bool,
}
