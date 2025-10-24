use dioxus::prelude::*;

use nekit_dev::routes::App;

fn main() -> Result<()> {
    dioxus::launch(App);

    Ok(())
}
