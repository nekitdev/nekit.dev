use std::borrow::Cow;

use maud::{html, Markup};

use crate::templates::base::{base, head, HeadContext};

pub const TITLE: &str = "Home";
pub const DESCRIPTION: &str = "Building awesome software.";

pub fn content() -> Markup {
    html! {
        div class="px-4 sm:px-6 md:px-8" {
            header class="
                relative
                pt-6
                lg:pt-8
                flex
                items-center
                justify-between
                text-neutral-700
                dark:text-neutral-200
                leading-6
            " {
                a class="text-neutral-900 dark:text-white text-2xl lg:text-3xl font-mono" href="#" {
                    "nekitdev"
                }

                div class="flex space-x-8" {
                    div class="hidden md:flex space-x-8" {
                        a class="text-youtube" href="/youtube" {
                            i class="fa-brands fa-youtube w-auto h-10" {}
                        }
                        a class="text-reddit" href="/reddit" {
                            i class="fa-brands fa-reddit-alien w-auto h-10" {}
                        }
                        a class="text-neutral-900 dark:text-white" href="/x" {
                            i class="fa-brands fa-x-twitter w-auto h-10" {}
                        }
                        a class="text-telegram" href="/telegram" {
                            i class="fa-brands fa-telegram w-auto h-10" {}
                        }
                        a class="text-discord" href="/discord" {
                            i class="fa-brands fa-discord w-auto h-10" {}
                        }
                        a class="text-keybase" href="/keybase" {
                            i class="fa-brands fa-keybase w-auto h-10" {}
                        }
                    }
                    a class="text-netural-900 dark:text-white" href="/github" {
                        i class="fa-brands fa-github w-auto h-10" {}
                    }
                    a class="
                        transition
                        ease-in-out
                        text-neutral-900
                        dark:text-white
                        text-xl
                        font-mono
                        bg-white
                        dark:bg-neutral-900
                        focus:outline-none
                        border-neutral-900
                        dark:border-white
                        hover:border-melody-blue
                        hover:text-melody-blue
                        dark:hover:border-melody-purple
                        dark:hover:text-melody-purple
                        border-2
                        h-10
                        px-4
                        rounded-lg
                        w-full
                        sm:w-auto
                        flex
                        items-center
                        justify-center
                    " href="/funding" {
                        "Funding"
                    }
                }
            }

            section class="relative max-w-5xl pt-16 sm:pt-20 lg:pt-24" {
                h1 class="text-5xl lg:text-8xl font-mono" {
                    "Building " span class="
                        text-transparent
                        bg-clip-text
                        bg-gradient-to-b
                        from-melody-purple
                        to-melody-blue
                    " {
                        "awesome"
                    } " software."
                }

                div class="mt-6 text-xl lg:text-2xl text-neutral-700 dark:text-neutral-500 font-mono" {
                    b { "Nikita Tikhonov" } ", software developer from Moscow, Russia."
                }
            }
        }
    }
}

pub fn index() -> Markup {
    base(
        head(&HeadContext::new(
            Cow::Borrowed(TITLE),
            Cow::Borrowed(DESCRIPTION),
        )),
        content(),
    )
}
