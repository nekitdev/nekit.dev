use anyhow::Result;
use clap::Parser;
use nekit_battery::app::App;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}
