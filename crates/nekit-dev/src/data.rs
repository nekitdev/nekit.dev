use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str, non_empty_string};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Link {
    pub name: NonEmptyString,
    pub path: NonEmptyString,
}

impl Link {
    pub const fn new(name: NonEmptyString, path: NonEmptyString) -> Self {
        Self { name, path }
    }

    pub fn channel() -> Self {
        Self::new(non_empty_string!("channel"), non_empty_string!("/channel"))
    }

    pub fn bluesky() -> Self {
        Self::new(non_empty_string!("bluesky"), non_empty_string!("/bluesky"))
    }

    pub fn reddit() -> Self {
        Self::new(non_empty_string!("reddit"), non_empty_string!("/reddit"))
    }

    pub fn youtube() -> Self {
        Self::new(non_empty_string!("youtube"), non_empty_string!("/youtube"))
    }

    pub fn telegram() -> Self {
        Self::new(
            non_empty_string!("telegram"),
            non_empty_string!("/telegram"),
        )
    }

    pub fn discord() -> Self {
        Self::new(non_empty_string!("discord"), non_empty_string!("/discord"))
    }

    pub fn email() -> Self {
        Self::new(
            non_empty_string!("email"),
            non_empty_string!("/email/nekit"),
        )
    }

    pub fn chat() -> Self {
        Self::new(non_empty_string!("chat"), non_empty_string!("/chat"))
    }

    pub fn github() -> Self {
        Self::new(non_empty_string!("github"), non_empty_string!("/github"))
    }

    pub fn dualizzzm() -> Self {
        Self::new(
            non_empty_string!("dualizzzm"),
            non_empty_string!("/dualizzzm"),
        )
    }

    pub fn lyrichar() -> Self {
        Self::new(
            non_empty_string!("lyrichar"),
            non_empty_string!("/lyrichar"),
        )
    }

    pub fn jiyu() -> Self {
        Self::new(non_empty_string!("jiyu"), non_empty_string!("/jiyu"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    pub name: NonEmptyString,
    pub links: Vec<Link>,
}

impl Group {
    pub const fn new(name: NonEmptyString, links: Vec<Link>) -> Self {
        Self { name, links }
    }

    pub fn social() -> Self {
        Self::new(
            non_empty_string!("social"),
            vec![
                Link::channel(),
                Link::bluesky(),
                Link::reddit(),
                Link::youtube(),
            ],
        )
    }

    pub fn contact() -> Self {
        Self::new(
            non_empty_string!("contact"),
            vec![
                Link::telegram(),
                Link::discord(),
                Link::email(),
                Link::chat(),
            ],
        )
    }

    pub fn code() -> Self {
        Self::new(
            non_empty_string!("code"),
            vec![
                Link::github(),
                Link::dualizzzm(),
                Link::lyrichar(),
                Link::jiyu(),
            ],
        )
    }

    pub fn all() -> Vec<Self> {
        vec![Self::social(), Self::contact(), Self::code()]
    }
}

pub const ALPHA: char = 'α';
pub const BETA: char = 'β';
pub const OMEGA: char = 'ω';

pub const PROGRESS: &NonEmptyStr = const_non_empty_str!("work in progress");
pub const UNSTABLE: &NonEmptyStr = const_non_empty_str!("unstable");
pub const STABLE: &NonEmptyStr = const_non_empty_str!("stable");

pub type StaticNonEmptyStr = &'static NonEmptyStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            Self::system(),
            Self::tag(),
        ]
    }
}
