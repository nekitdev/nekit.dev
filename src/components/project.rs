use dioxus::prelude::*;

#[component]
pub fn Project(title: String, description: String, name: String) -> Element {
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
                        { title }
                    }
                    a {
                        span {
                            class: "ml-auto fa-brands fa-github text-4xl",
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
