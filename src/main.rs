mod build;
mod primitives;
mod print;
mod quit;

fn main() {
    let mocked_raw_input = String::from("830609000\n602000900\n500007001\n000000069\n390208000\n050406023\n920070500\n000005400\n010900007");

    let sudoku = build::run(mocked_raw_input);

    if let Err(error) = &sudoku {
        quit::exit_with_error(error);
    }

    print::run(&sudoku.unwrap());
}
