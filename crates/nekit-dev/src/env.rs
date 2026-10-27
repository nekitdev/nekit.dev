use std::{env, path::PathBuf};

pub const BLOG: &str = "NEKIT_BLOG";
pub const DEFAULT_BLOG: &str = "/blog/current";

pub fn blog() -> PathBuf {
    if let Some(blog) = env::var_os(BLOG) {
        PathBuf::from(blog)
    } else {
        PathBuf::from(DEFAULT_BLOG)
    }
}
