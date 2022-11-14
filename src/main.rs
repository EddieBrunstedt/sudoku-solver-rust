use lib::build_sudoku_grid::create_grid_from_string;
use lib::print::print_grid;
use lib::quit::exit_with_error;

fn main() {
    let mocked_raw_input = String::from("830609000\n602000900\n500007001\n000000069\n390208000\n050406023\n920070500\n000005400\n010900007");

    let input_sudoku = create_grid_from_string(mocked_raw_input);

    if let Err(error) = &input_sudoku {
        exit_with_error(error);
    }

    print_grid(&input_sudoku.unwrap());
}
