use crate::primitives::{Cell, Errors, SudokuGrid};

pub fn run(string: String) -> Result<SudokuGrid, Errors> {
    let trimmed_string = trim_string(string);

    if trimmed_string.len() != 81 {
        return Err(Errors::InvalidStringLength);
    };

    let vector_grid = build_vector_grid(trimmed_string);

    return Ok(vector_grid);
}

fn build_vector_grid(string: String) -> SudokuGrid {
    let mut result = [(); 9].map(|_| [(); 9].map(|_| Cell::default()));

    for row_iteration in 0..9 {
        let start_index = 9 * row_iteration;
        let end_index = 9 * (row_iteration + 1);
        let row_slice = &string[start_index..end_index];

        let mut column_iteration = 0;

        for c in row_slice.chars() {
            let value_from_input: bool;
            let numeric_value = c.to_digit(10).unwrap();

            if numeric_value == 0 {
                value_from_input = false;
            } else {
                value_from_input = true;
            }

            let cell = Cell {
                value: numeric_value,
                from_input: value_from_input,
            };

            result[row_iteration][column_iteration] = cell;

            column_iteration += 1;
        }
    }

    return result;
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
