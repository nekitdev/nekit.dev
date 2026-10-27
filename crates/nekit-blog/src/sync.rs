use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use tokio::sync::watch;

use crate::{
    blog::Blog,
    load::{Failure, load},
};

pub const NAME: &str = "blog";

pub const INTERVAL: Duration = Duration::from_secs(30);

pub type Shared = Arc<Blog>;

const SPAWN: &str = "failed to spawn blog watch thread";

fn resolve<L: AsRef<Path>>(link: L) -> Option<PathBuf> {
    canonicalize(link).ok()
}

type FailureRef<'f> = &'f Failure;

fn report<'f, F: IntoIterator<Item = FailureRef<'f>>>(failures: F) {
    failures.into_iter().for_each(|failure| {
        eprintln!("{failure}");
    });
}

fn reload<T: AsRef<Path>>(target: T) -> Option<Blog> {
    match load(target) {
        Ok(load) => {
            let (blog, failures) = load.split();

            report(failures.iter());

            Some(blog)
        }
        Err(error) => {
            eprintln!("{error}");

            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Watch {
    receiver: watch::Receiver<Shared>,
}

impl Watch {
    pub fn spawn(link: PathBuf) -> Self {
        Self::spawn_with(link, INTERVAL)
    }

    pub fn spawn_with(link: PathBuf, interval: Duration) -> Self {
        let mut target = resolve(link.as_path());

        let initial = target.as_deref().and_then(reload).unwrap_or_default();

        let shared = Arc::new(initial);

        let (sender, receiver) = watch::channel(shared);

        thread::Builder::new()
            .name(NAME.to_owned())
            .spawn(move || {
                loop {
                    thread::sleep(interval);

                    // nobody is listening
                    if sender.is_closed() {
                        break;
                    }

                    let current = resolve(link.as_path());

                    if current == target {
                        continue;
                    }

                    let Some(path) = current.as_deref() else {
                        continue; // link was not resolved; keep serving previous blog
                    };

                    if let Some(blog) = reload(path) {
                        let shared = Arc::new(blog);

                        sender.send_replace(shared);

                        target = current;
                    }
                }
            })
            .expect(SPAWN);

        Self { receiver }
    }

    pub fn blog(&self) -> Shared {
        self.receiver.borrow().clone()
    }

    pub fn subscribe(&self) -> watch::Receiver<Shared> {
        self.receiver.clone()
    }
}
