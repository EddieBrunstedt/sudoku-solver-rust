use crate::primitives::SudokuGrid;
use std::collections::HashMap;

pub fn run(mut sudoku: SudokuGrid) -> SudokuGrid {
    let mut cursor = Cursor::new();

    'outer: loop {
        if sudoku[cursor.row][cursor.col].from_input {
            if cursor.col == 8 && cursor.row == 8 {
                break 'outer;
            };
            cursor.move_along();
        } else {
            'inner: loop {
                if sudoku[cursor.row][cursor.col].value == 9 {
                    sudoku[cursor.row][cursor.col].value = 0;
                    cursor.move_along();
                    continue 'outer;
                } else {
                    sudoku[cursor.row][cursor.col].value += 1;
                }

                if is_conflict_at_cursor(&cursor, &sudoku) {
                    if sudoku[cursor.row][cursor.col].value == 9 {
                        sudoku[cursor.row][cursor.col].value = 0;
                        cursor.move_backward();
                        continue 'outer;
                    } else {
                        continue 'inner;
                    }
                } else {
                    if cursor.col == 8 && cursor.row == 8 {
                        break 'outer;
                    };
                    cursor.move_forward();
                    continue 'outer;
                }
            }
        }
    }

    return sudoku;
}

fn is_conflict_at_cursor(cursor: &Cursor, sudoku: &SudokuGrid) -> bool {
    return has_duplicates(get_row_nums(cursor, sudoku))
        || has_duplicates(get_col_nums(cursor, sudoku))
        || has_duplicates(get_box_nums(cursor, sudoku))
        || false;
}

fn has_duplicates(nums: Vec<u32>) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    for i in nums {
        let v = map.entry(i).or_insert(0);
        *v += 1;
        if *v > 1 {
            return true;
        }
    }

    return false;
}

fn get_row_nums(cursor: &Cursor, sudoku: &SudokuGrid) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    for i in 0..9 {
        if sudoku[cursor.row][i].value == 0 {
            continue;
        } else {
            result.push(sudoku[cursor.row][i].value);
        }
    }

    return result;
}

fn get_col_nums(cursor: &Cursor, sudoku: &SudokuGrid) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    for i in 0..9 {
        if sudoku[i][cursor.col].value == 0 {
            continue;
        } else {
            result.push(sudoku[i][cursor.col].value);
        }
    }

    return result;
}

fn get_box_nums(cursor: &Cursor, sudoku: &SudokuGrid) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let row_start = cursor.row / 3 * 3;
    let col_start = cursor.col / 3 * 3;

    for row_iteration in 0..3 {
        for col_iteration in 0..3 {
            if sudoku[row_start + row_iteration][col_start + col_iteration].value == 0 {
                continue;
            } else {
                result.push(sudoku[row_start + row_iteration][col_start + col_iteration].value);
            }
        }
    }

    return result;
}

#[derive(Debug)]
struct Cursor {
    col: usize,
    row: usize,
    current_direction: CursorDirection,
}

#[derive(Debug)]
enum CursorDirection {
    Forward,
    Backward,
}

trait MoveCursor {
    fn move_along(&mut self);
    fn move_backward(&mut self);
    fn move_forward(&mut self);
}

trait CreateCursor {
    fn new() -> Cursor;
}

impl CreateCursor for Cursor {
    fn new() -> Cursor {
        return Cursor {
            col: 0,
            row: 0,
            current_direction: CursorDirection::Forward,
        };
    }
}

impl MoveCursor for Cursor {
    fn move_along(&mut self) {
        match self.current_direction {
            CursorDirection::Forward => {
                return self.move_forward();
            }
            CursorDirection::Backward => {
                return self.move_backward();
            }
        }
    }

    fn move_backward(&mut self) {
        self.current_direction = CursorDirection::Backward;

        if self.col == 0 {
            self.col = 8;
            self.row -= 1;
        } else {
            self.col -= 1;
        }
    }

    fn move_forward(&mut self) {
        self.current_direction = CursorDirection::Forward;

        if self.col == 8 {
            self.col = 0;
            self.row += 1;
        } else {
            self.col += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_duplicates() {
        assert_eq!(has_duplicates(vec![1, 2, 3, 4]), false);
        assert_eq!(has_duplicates(vec![1, 2, 3, 1]), true);
        assert_eq!(has_duplicates(vec![1, 1, 3, 4]), true);
        assert_eq!(has_duplicates(vec![]), false);
        assert_eq!(has_duplicates(vec![1, 1, 1, 1, 1]), true);
    }

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
