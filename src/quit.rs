use crate::primitives::Errors;

pub fn exit_with_error(error: &Errors) {
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
