#import "@preview/modern-cv:0.10.0": *

#show: resume.with(
  author: (
    firstname: "Nikita",
    lastname: "Tikhonov",
    birth: "13/01/2005",
    email: "nekit@nekit.dev",
    phone: "+7 (926) 126-22-06",
    homepage: "https://nekit.dev/",
    github: "nekitdev",
    bluesky: "nekit.dev",
    telegram: "nekitdev",
    twitter: "nekitdev",
    positions: (
      "Software Engineer",
      "Fullstack Developer",
    ),
  ),
  show-footer: false,
  profile-picture: none,
)

= Projects

#resume-entry(
  title: "Lyrics at the speed of thought",
  location: [#github-link("lyrichar/lyrichar")],
  date: "June 2026 - Present",
  description: "Designer/Developer",
)

#resume-item[
  - Designing and developing cross-platform application for writing poems and lyrics.
  - Using Rust and Dioxus on the frontend, Axum and SeaORM (PostgreSQL) on the backend.
]

#resume-entry(
  title: "Obtaining ownership in Rust",
  location: [#github-link("nekitdev/ownership")],
  date: "August 2025 - Present",
  description: "Designer/Developer",
)

#resume-item[
  - Designed and implemented derive macros for obtaining ownership in Rust.
  - Wrote documentation and tests and published the library on crates.io.
]

#resume-entry(
  title: "Graph data structures and algorithms",
  location: [#github-link("nekitdev/graphs")],
  date: "June 2025 - Present",
  description: "Designer/Developer",
)

#resume-item[
  - Designed and implemented graph data structures and algorithms in Rust.
  - Wrote documentation and tests and published the library on GitHub.
]

#resume-entry(
  title: "Refinement types",
  location: [#github-link("nekitdev/refining")],
  date: "February 2025 - Present",
  description: "Designer/Developer",
)

#resume-item[
  - Designed and implemented refinement types for Rust.
  - Wrote documentation and tests and published the library on crates.io.
]

#resume-entry(
  title: "Personal website",
  location: [#github-link("nekitdev/nekit.dev")],
  date: "September 2024 - Present",
  description: "Designer/Developer",
)

#resume-item[
  - Designed and developed personal website using Rust and Dioxus.
  - Implemented responsive design and optimized performance.
]

#resume-entry(
  title: "Building changelogs from fragments",
  location: [#github-link("nekitdev/changelogging")],
  date: "May 2024 - Present",
  description: "Designer/Developer",
)

#resume-item[
  - Designed and developed CLI tool for building changelogs from fragments in Rust.
  - Wrote documentation and tests and published the tool on crates.io.
]

= Skills

#resume-skill-grid(
  categories-with-values: (
    "Programming Languages": (
      strong("Rust"),
      strong("Python"),
      "TypeScript",
      "SQL",
      "C#",
    ),
    "Spoken Languages": (
      strong("Russian"),
      strong("English"),
    ),
  ),
)

= Education

#resume-entry(
  date: "2024 - Present",
  location: "MIREA - Russian Technological University",
  title: "B.S. Information Systems and Technologies",
  description: "Fullstack Development",
)
