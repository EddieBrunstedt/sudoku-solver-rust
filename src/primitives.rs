use colored::{ColoredString, Colorize};
use std::default::Default;

pub type SudokuGrid = [[Cell; 9]; 9];

#[derive(Default, Debug)]
pub struct Cell {
    pub value: u32,
    pub from_input: bool,
}

impl Cell {
    pub fn create_colorized_string(&self) -> ColoredString {
        if self.value == 0 {
            return " ".to_string().bold();
        };

        match self.from_input {
            true => return self.value.to_string().bold().green(),
            false => return self.value.to_string().bold().red(),
        }
    }
}

#[derive(Debug)]
pub enum Errors {
    InvalidStringLength,
}

pub struct Cursor {
    pub col: usize,
    pub row: usize,
    pub current_direction: CursorDirection,
}

#[derive(Debug)]
pub enum CursorDirection {
    Forward,
    Backward,
}

impl Cursor {
    pub fn new() -> Cursor {
        return Cursor {
            col: 0,
            row: 0,
            current_direction: CursorDirection::Forward,
        };
    }

    pub fn move_along(&mut self) {
        match self.current_direction {
            CursorDirection::Forward => {
                return self.move_forward();
            }
            CursorDirection::Backward => {
                return self.move_backward();
            }
        }
    }

    pub fn move_backward(&mut self) {
        self.current_direction = CursorDirection::Backward;

        (self.row, self.col) = if self.col == 0 {
            (self.row - 1, 8)
        } else {
            (self.row, self.col - 1)
        };
    }

    pub fn move_forward(&mut self) {
        self.current_direction = CursorDirection::Forward;

        (self.row, self.col) = if self.col == 8 {
            (self.row + 1, 0)
        } else {
            (self.row, self.col + 1)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_cursor() {
        let mut cursor = Cursor::new();
        assert_eq!((cursor.row, cursor.col), (0, 0));
        for _ in 0..7 {
            cursor.move_forward();
        }
        assert_eq!((cursor.row, cursor.col), (0, 7));
        for _ in 0..24 {
            cursor.move_along();
        }
        assert_eq!((cursor.row, cursor.col), (3, 4));
        for _ in 0..5 {
            cursor.move_backward();
        }
        assert_eq!((cursor.row, cursor.col), (2, 8));
        for _ in 0..5 {
            cursor.move_along();
        }
        assert_eq!((cursor.row, cursor.col), (2, 3));
    }
}
