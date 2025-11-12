use dioxus::prelude::*;

#[component]
pub fn Project(
    title: String,
    description: String,
    repository: String,
    documentation: String,
) -> Element {
    rsx! {
        li {
            class: "
                max-w-sm
                p-1
                hover:bg-gradient-to-b hover:from-melody-purple hover:to-melody-blue
                bg-neutral-200 dark:bg-neutral-700
                rounded-lg
            ",
            section {
                class: "
                    p-4
                    h-full
                    rounded-lg
                    bg-neutral-100 dark:bg-neutral-800
                ",
                div {
                    class: "mb-2 flex items-center justify-center gap-4",
                    h3 {
                        class: "mr-auto font-bold text-xl",
                        "{title}"
                    }
                    div {
                        class: "ml-auto flex gap-4",
                        a {
                            href: repository,
                            aria_label: "{title} GitHub",
                            span {
                                class: "
                                    fa-brands fa-github
                                    hover:text-melody-blue dark:hover:text-melody-purple
                                    text-4xl
                                ",
                            }
                        }
                        a {
                            href: documentation,
                            aria_label: "{title} Documentation",
                            span {
                                class: "
                                    fa-solid fa-book
                                    hover:text-melody-blue dark:hover:text-melody-purple
                                    text-4xl
                                ",
                            }
                        }
                    }
                }

                div {
                    class: "h-px bg-gradient-to-r from-melody-purple to-melody-blue",
                    role: "separator",
                }

                p {
                    class: "mt-2",
                    { description }
                }
            }
        }
    }
}
