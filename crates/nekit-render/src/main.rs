use anyhow::Result;
use clap::Parser;
use nekit_render::app::App;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}
