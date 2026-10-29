pub mod battery;
pub mod blog;
pub mod components;
pub mod data;
pub mod host;
pub mod hydrated;
pub mod keys;
pub mod redirects;
pub mod routes;
pub mod theme;
pub mod times;
pub mod urls;
pub mod views;

cfg_select! {
    feature = "web" => {
        pub mod browser;
    }
    _ => {
        pub mod env;
        pub mod sync;
        pub mod monitor;
    }
}
