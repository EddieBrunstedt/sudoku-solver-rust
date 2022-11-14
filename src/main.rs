use colored::{ColoredString, Colorize};

#[derive(Debug)]
enum Errors {
    InvalidStringLength,
}

struct Cell {
    value: u32,
    from_input: bool,
}

trait CreateColorizedString {
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

type SudokuGrid = Vec<Vec<Cell>>;

fn main() {
    let mocked_raw_input = String::from("830609000\n602000900\n500007001\n000000069\n390208000\n050406023\n920070500\n000005400\n010900007");

    let input_sudoku = create_input_sudoku(mocked_raw_input);

    if let Err(error) = &input_sudoku {
        exit_with_error(error);
    }

    print_grid(&input_sudoku.unwrap());
}

fn exit_with_error(error: &Errors) {
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

fn print_grid(grid: &SudokuGrid) {
    let mut row_count: u8 = 0;

    for row in grid {
        if row_count == 0 {
            println!("{}", "╔═══════════╦═══════════╦═══════════╗".red());
        } else if row_count % 3 == 0 {
            println!("{}", "╠═══════════╬═══════════╬═══════════╣".red());
        }

        let mut cell_count: u8 = 0;

        for cell in row {
            if cell_count % 3 == 0 {
                print!("{}", "║".red());
            }

            if [0, 3, 6].contains(&cell_count) {
                print!(" {} {}", cell.create_colorized_string(), "│".red())
            } else if [1, 4, 7].contains(&cell_count) {
                print!(" {} ", cell.create_colorized_string())
            } else if [2, 5, 8].contains(&cell_count) {
                print!("{} {} ", "│".red(), cell.create_colorized_string())
            }

            if cell_count == 8 {
                print!("{}", "║".red());
            }

            cell_count += 1;
        }

        if ![2, 5, 8].contains(&row_count) {
            println!();
            println!("{}", "║───┼───┼───║───┼───┼───║───┼───┼───║".red());
        } else {
            println!();
        }

        if row_count == 8 {
            println!("{}", "╚═══════════╩═══════════╩═══════════╝".red());
        }

        row_count += 1;
    }
}

fn validate_string_length(string: String) -> Result<String, Errors> {
    if string.len() != 81 {
        return Err(Errors::InvalidStringLength);
    };

    return Ok(string);
}

fn trim_string(string: String) -> String {
    let mut result: String = String::new();

    for c in string.trim().chars() {
        if c.is_ascii_digit() {
            result.push(c);
        }
    }

    return result;
}

fn create_input_sudoku(string: String) -> Result<SudokuGrid, Errors> {
    let validated_input = validate_string_length(trim_string(string))?;
    let mut result: SudokuGrid = vec![];

    for iteration in 0..9 {
        let mut row_vector = vec![];

        let start_index = 9 * iteration;
        let end_index = 9 * (iteration + 1);

        let slice = &validated_input[start_index..end_index];

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

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim() {
        assert_eq!(trim_string(String::from("")), String::from(""));
        assert_eq!(
            trim_string(String::from(
                " 1  2\n 3 \n\n\n | \\ . , &*(%##@!%) asdf 45 \n "
            )),
            String::from("12345")
        );
    }
}
