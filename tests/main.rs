use lib::build_sudoku_grid::trim_string;

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
