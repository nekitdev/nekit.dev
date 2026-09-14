use anyhow::Result;
use clap::Parser;
use nekit_serve::app::App;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}
