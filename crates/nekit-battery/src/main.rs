use anyhow::Result;
use clap::Parser;
use nekit_battery::app::App;
use pid1::relaunch_if_pid1;

fn main() -> Result<()> {
    relaunch_if_pid1().expect("failed to relaunch");

    App::parse().run()?;

    Ok(())
}
