use crate::primitives::CreateColorizedString;
use crate::primitives::SudokuGrid;
use colored::Colorize;

pub fn run(grid: &SudokuGrid) {
    let mut row_count: u8 = 0;

    for row in grid {
        if row_count == 0 {
            println!("{}", "╔═══════════╦═══════════╦═══════════╗".bright_black());
        } else if row_count % 3 == 0 {
            println!("{}", "╠═══════════╬═══════════╬═══════════╣".bright_black());
        }

        let mut cell_count: u8 = 0;

        for cell in row {
            if cell_count % 3 == 0 {
                print!("{}", "║".bright_black());
            }

            if [0, 3, 6].contains(&cell_count) {
                print!(" {} {}", cell.create_colorized_string(), "│".bright_black())
            } else if [1, 4, 7].contains(&cell_count) {
                print!(" {} ", cell.create_colorized_string())
            } else if [2, 5, 8].contains(&cell_count) {
                print!("{} {} ", "│".bright_black(), cell.create_colorized_string())
            }

            if cell_count == 8 {
                print!("{}", "║".bright_black());
            }

            cell_count += 1;
        }

        if ![2, 5, 8].contains(&row_count) {
            println!();
            println!("{}", "║───┼───┼───║───┼───┼───║───┼───┼───║".bright_black());
        } else {
            println!();
        }

        if row_count == 8 {
            println!("{}", "╚═══════════╩═══════════╩═══════════╝".bright_black());
        }

        row_count += 1;
    }
}
