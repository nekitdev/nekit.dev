use nekit_dev::routes::App;

fn main() {
    #[cfg(feature = "server")]
    pid1::relaunch_if_pid1().expect("failed to relaunch");

    dioxus::launch(App);
}
