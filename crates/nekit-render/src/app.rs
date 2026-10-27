use std::path::PathBuf;

use clap::Parser;

use crate::file::{Error, render_check, render_from_to, render_print};

#[derive(Debug, Parser)]
pub struct App {
    #[arg(
        short = 'p',
        long,
        env = "NEKIT_RENDER_PACKAGES",
        value_name = "DIRECTORY"
    )]
    pub packages: Option<PathBuf>,

    #[arg(short = 'o', long, env = "NEKIT_RENDER_OUTPUT", value_name = "PATH")]
    pub output: Option<PathBuf>,

    #[arg(short = 'c', long, env = "NEKIT_RENDER_CHECK", action)]
    pub check: bool,

    #[arg(value_name = "FILE")]
    pub inputs: Vec<PathBuf>,
}

impl App {
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }

    pub fn is_check(&self) -> bool {
        self.check
    }

    pub fn run(&self) -> Result<(), Error> {
        let packages = self.packages.as_deref();

        let output = self.output.as_deref();

        let inputs = self.inputs.as_slice();

        let check = self.is_check();

        if let [one, rest @ ..] = inputs {
            if rest.is_empty() {
                if check {
                    render_check(one, packages)?;
                } else if let Some(path) = output {
                    render_from_to(one, path, packages)?;
                } else {
                    render_print(one, packages)?;
                }
            } else {
                todo!()
            }
        };

        Ok(())
    }
}
