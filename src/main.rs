use clap::Parser;
use miette::Result;
use nekit_dev::app::App;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}
