use std::sync::LazyLock;

use nekit_blog::sync::{Shared, Watch};

use crate::env;

pub fn watch() -> Watch {
    Watch::spawn(env::blog())
}

static WATCH: LazyLock<Watch> = LazyLock::new(watch);

pub fn blog() -> Shared {
    WATCH.blog()
}
