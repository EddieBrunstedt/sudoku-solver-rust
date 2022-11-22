mod build;
mod primitives;
mod print;
mod solve;

use build::run as build;
use print::run as print;
use solve::run as solve;

fn main() {
    let mocked_raw_input = String::from("530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079");

    let initial_sudoku = build(mocked_raw_input).expect("Error building sudoku");

    print(&initial_sudoku);

    let solved_sudoku = solve(initial_sudoku);

    print(&solved_sudoku);
}
