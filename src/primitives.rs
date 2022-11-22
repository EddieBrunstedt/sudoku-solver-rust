use colored::{ColoredString, Colorize};
use std::default::Default;

pub type SudokuGrid = [[Cell; 9]; 9];

#[derive(Default, Debug)]
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
            false => return self.value.to_string().bold().red(),
        }
    }
}

#[derive(Debug)]
pub enum Errors {
    InvalidStringLength,
}
