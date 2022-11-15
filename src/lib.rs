pub mod build_sudoku_grid {
    use crate::types::{Cell, Errors, SudokuGrid};

    pub fn create_grid_from_string(string: String) -> Result<SudokuGrid, Errors> {
        let trimmed_string = trim_string(string);

        if trimmed_string.len() != 81 {
            return Err(Errors::InvalidStringLength);
        };

        let vector_grid = build_vector_grid(trimmed_string);

        return Ok(vector_grid);
    }

    fn build_vector_grid(string: String) -> SudokuGrid {
        let mut result: SudokuGrid = vec![];

        for iteration in 0..9 {
            let mut row_vector = vec![];

            let start_index = 9 * iteration;
            let end_index = 9 * (iteration + 1);

            let slice = &string[start_index..end_index];

            for c in slice.chars() {
                let value_from_input: bool;
                let numeric_value = c.to_digit(10).unwrap();

                if numeric_value == 0 {
                    value_from_input = true;
                } else {
                    value_from_input = false;
                }

                let cell = Cell {
                    value: numeric_value,
                    from_input: value_from_input,
                };

                row_vector.push(cell);
            }

            result.push(row_vector);
        }

        return result;
    }

    pub fn trim_string(string: String) -> String {
        let mut result: String = String::new();

        for c in string.trim().chars() {
            if c.is_ascii_digit() {
                result.push(c);
            }
        }

        return result;
    }
}

pub mod print {
    use crate::types::{CreateColorizedString, SudokuGrid};
    use colored::Colorize;

    pub fn print_grid(grid: &SudokuGrid) {
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
}

pub mod types {
    use colored::{ColoredString, Colorize};

    pub type SudokuGrid = Vec<Vec<Cell>>;

    #[derive(Debug)]
    pub enum Errors {
        InvalidStringLength,
    }

    pub struct Cell {
        pub value: u32,
        pub from_input: bool,
    }

    pub trait CreateColorizedString {
        fn create_colorized_string(&self) -> ColoredString;
    }

    impl CreateColorizedString for Cell {
        fn create_colorized_string(&self) -> ColoredString {
            if self.value == 0 {
                return " ".to_string().bold();
            };

            match self.from_input {
                true => return self.value.to_string().bold().green(),
                false => return self.value.to_string().bold().cyan(),
            }
        }
    }
}

pub mod quit {
    use crate::types::Errors;

    pub fn exit_with_error(error: &Errors) {
        fn print_error_message(message: &str) {
            eprintln!("Error: {}", message);
        }

        match error {
            Errors::InvalidStringLength => {
                print_error_message("The amount of characters in input does not match expectation")
            }
        }

        std::process::exit(1);
    }
}
