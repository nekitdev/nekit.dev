use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str, non_empty_string};

pub const ALPHA: char = 'α';
pub const BETA: char = 'β';
pub const OMEGA: char = 'ω';

pub const PROGRESS: &NonEmptyStr = const_non_empty_str!("work in progress");
pub const UNSTABLE: &NonEmptyStr = const_non_empty_str!("unstable");
pub const STABLE: &NonEmptyStr = const_non_empty_str!("stable");

pub type StaticNonEmptyStr = &'static NonEmptyStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Progress,
    Unstable,
    Stable,
}

pub use Status::{Progress, Stable, Unstable};

impl Status {
    pub const fn symbol(self) -> char {
        match self {
            Self::Progress => ALPHA,
            Self::Unstable => BETA,
            Self::Stable => OMEGA,
        }
    }

    pub const fn label(self) -> StaticNonEmptyStr {
        match self {
            Self::Progress => PROGRESS,
            Self::Unstable => UNSTABLE,
            Self::Stable => STABLE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub status: Status,
    pub name: NonEmptyString,
    pub description: NonEmptyString,
    pub repository: NonEmptyString,
}

impl Project {
    pub const fn new(
        status: Status,
        name: NonEmptyString,
        description: NonEmptyString,
        repository: NonEmptyString,
    ) -> Self {
        Self {
            name,
            status,
            description,
            repository,
        }
    }

    pub fn changelogging() -> Self {
        Self::new(
            Stable,
            non_empty_string!("changelogging"),
            non_empty_string!("Building changelogs from fragments."),
            non_empty_string!("https://github.com/nekitdev/changelogging"),
        )
    }

    pub fn trait_aliases() -> Self {
        Self::new(
            Stable,
            non_empty_string!("trait-aliases"),
            non_empty_string!("Trait aliases."),
            non_empty_string!("https://github.com/nekitdev/trait-aliases"),
        )
    }

    pub fn non_empty_str() -> Self {
        Self::new(
            Stable,
            non_empty_string!("non-empty-str"),
            non_empty_string!("Non-empty strings."),
            non_empty_string!("https://github.com/nekitdev/non-empty-str"),
        )
    }

    pub fn otp_std() -> Self {
        Self::new(
            Unstable,
            non_empty_string!("otp-std"),
            non_empty_string!("Generating and checking one-time passwords."),
            non_empty_string!("https://github.com/nekitdev/otp-std"),
        )
    }

    pub fn ownership() -> Self {
        Self::new(
            Unstable,
            non_empty_string!("ownership"),
            non_empty_string!("Obtaining ownership."),
            non_empty_string!("https://github.com/nekitdev/ownership"),
        )
    }

    pub fn refining() -> Self {
        Self::new(
            Unstable,
            non_empty_string!("refining"),
            non_empty_string!("Refinement types."),
            non_empty_string!("https://github.com/nekitdev/refining"),
        )
    }

    pub fn jiyu() -> Self {
        Self::new(
            Progress,
            non_empty_string!("jiyu"),
            non_empty_string!("Freedom of managing proxies."),
            non_empty_string!("https://github.com/jiyu-sh/jiyu"),
        )
    }

    pub fn lyrichar() -> Self {
        Self::new(
            Progress,
            non_empty_string!("lyrichar"),
            non_empty_string!("Lyrics at the speed of thought."),
            non_empty_string!("https://github.com/lyrichar/lyrichar"),
        )
    }

    pub fn graphs() -> Self {
        Self::new(
            Progress,
            non_empty_string!("graphs"),
            non_empty_string!("Graph data structures and algorithms."),
            non_empty_string!("https://github.com/nekitdev/graphs"),
        )
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::changelogging(),
            Self::trait_aliases(),
            Self::non_empty_str(),
            Self::otp_std(),
            Self::ownership(),
            Self::refining(),
            Self::jiyu(),
            Self::lyrichar(),
            Self::graphs(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fact {
    pub title: NonEmptyString,
    pub value: NonEmptyString,
}

pub fn age(value: i16) -> NonEmptyString {
    // SAFETY: integer converted to string is always non-empty
    unsafe { NonEmptyString::new_unchecked(value.to_string()) }
}

impl Fact {
    pub const fn new(title: NonEmptyString, value: NonEmptyString) -> Self {
        Self { title, value }
    }

    pub fn name() -> Self {
        Self::new(
            non_empty_string!("Name"),
            non_empty_string!("Nikita Tikhonov"),
        )
    }

    pub fn pronouns() -> Self {
        Self::new(non_empty_string!("Pronouns"), non_empty_string!("he/they"))
    }

    pub fn age(value: i16) -> Self {
        Self::new(non_empty_string!("Age"), age(value))
    }

    pub fn location() -> Self {
        Self::new(
            non_empty_string!("Location"),
            non_empty_string!("Moscow, Russia"),
        )
    }

    pub fn spoken() -> Self {
        Self::new(
            non_empty_string!("Spoken"),
            non_empty_string!("Russian, English"),
        )
    }

    pub fn system() -> Self {
        Self::new(non_empty_string!("System"), non_empty_string!("NixOS"))
    }

    pub fn tag() -> Self {
        Self::new(non_empty_string!("Tag"), non_empty_string!("nekitdev"))
    }

    pub fn all(age: i16) -> Vec<Self> {
        vec![
            Self::name(),
            Self::pronouns(),
            Self::age(age),
            Self::location(),
            Self::spoken(),
            Self::system(),
            Self::tag(),
        ]
    }
}
