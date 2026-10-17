use dioxus::prelude::*;

pub fn provide_hydrated() -> Hydrated {
    let mut hydrated = use_signal(|| false);

    use_effect(move || hydrated.set(true));

    use_context_provider(|| Hydrated::construct(hydrated.into()))
}

#[derive(Debug, Clone, Copy)]
pub struct Hydrated {
    pub signal: ReadSignal<bool>,
}

impl Hydrated {
    pub fn construct(signal: ReadSignal<bool>) -> Self {
        Self { signal }
    }

    pub fn get(&self) -> bool {
        (self.signal)()
    }
}

pub fn use_hydrated() -> Hydrated {
    use_context()
}
