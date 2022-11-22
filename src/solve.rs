use crate::primitives::{CreateCursor, Cursor, MoveCursor, SudokuGrid};
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

                if conflict_at_cursor(&cursor, &sudoku) {
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

fn conflict_at_cursor(cursor: &Cursor, sudoku: &SudokuGrid) -> bool {
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
}
