use std::collections::BTreeMap;

use non_empty_str::NonEmptyStr;
use trait_aliases::trait_aliases;

use crate::{
    post::{Post, Summary},
    slug::Slug,
};

pub type PostMap = BTreeMap<Slug, Post>;

trait_aliases! {
    pub trait Posts<'p> = Iterator<Item = &'p Post>;
}

pub type Summaries = Vec<Summary>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blog {
    posts: PostMap,
}

impl Default for Blog {
    fn default() -> Self {
        Self::new()
    }
}

impl Blog {
    pub const fn new() -> Self {
        Self::with(BTreeMap::new())
    }

    pub const fn with(posts: PostMap) -> Self {
        Self { posts }
    }

    pub fn insert(&mut self, post: Post) -> Option<Post> {
        self.posts.insert(post.slug().clone(), post)
    }

    pub fn get<Q: AsRef<NonEmptyStr>>(&self, query: Q) -> Option<&Post> {
        self.posts.get(query.as_ref())
    }

    pub fn read<Q: AsRef<NonEmptyStr>>(&self, query: Q) -> Option<Post> {
        self.get(query).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.posts.is_empty()
    }

    pub fn count(&self) -> usize {
        self.posts.len()
    }

    pub fn posts(&self) -> impl Posts<'_> {
        self.posts.values()
    }

    pub fn published(&self) -> impl Posts<'_> {
        self.posts().filter(|post| post.is_published())
    }

    pub fn summaries(&self) -> Summaries {
        let mut summaries: Summaries = self
            .published()
            .map(|post| post.summary())
            .cloned()
            .collect();

        summaries.sort_by(|summary, other| summary.newest_first(other));

        summaries
    }
}

impl FromIterator<Post> for Blog {
    fn from_iter<I: IntoIterator<Item = Post>>(iterable: I) -> Self {
        let mut blog = Self::new();

        blog.extend(iterable);

        blog
    }
}

impl Extend<Post> for Blog {
    fn extend<I: IntoIterator<Item = Post>>(&mut self, iterable: I) {
        iterable.into_iter().for_each(|post| {
            self.insert(post);
        });
    }
}
