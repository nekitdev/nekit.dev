use nekit_dev::routes::App;

#[cfg(feature = "server")]
fn setup() {
    use nekit_dev::monitor;

    pid1::relaunch_if_pid1().expect("failed to relaunch");

    println!("starting monitor...");

    monitor::start();
}

fn main() {
    #[cfg(feature = "server")]
    setup();

    dioxus::launch(App);
}
