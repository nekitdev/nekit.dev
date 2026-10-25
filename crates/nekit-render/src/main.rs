use std::{
    io::{self, Read},
    process::ExitCode,
};

use nekit_render::{by::By, context::Context, markdown::render};

fn main() -> ExitCode {
    let mut string = String::new();

    io::stdin()
        .read_to_string(string.by_mut())
        .expect("failed to read input");

    let context = Context::builder().build();

    match render(string.as_str(), context) {
        Ok(html) => {
            println!("{html}");

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");

            ExitCode::FAILURE
        }
    }
}
