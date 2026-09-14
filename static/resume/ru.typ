#import "@preview/modern-cv:0.10.0": *

#show: resume.with(
  author: (
    firstname: "Никита",
    lastname: "Тихонов",
    birth: "13/01/2005",
    email: "nekit@nekit.dev",
    phone: "+7 (926) 126-22-06",
    homepage: "https://nekit.dev/",
    github: "nekitdev",
    bluesky: "nekit.dev",
    telegram: "nekitdev",
    twitter: "nekitdev",
    positions: (
      "Программист",
      "Фуллстек разработчик",
    ),
  ),
  show-footer: false,
  profile-picture: none,
)

= Проекты

#resume-entry(
  title: "Лирика со скоростью мысли",
  location: [#github-link("lyrichar/lyrichar")],
  date: "Июнь 2026 - Настоящее время",
  description: "Дизайнер/Разработчик",
)

#resume-item[
  - Дизайн и разработка кроссплатформенного приложения для написания стихов и текстов песен.
  - Использование Rust и Dioxus на фронтенде, Axum и SeaORM (PostgreSQL) на бэкенде.
]

#resume-entry(
  title: "Получение владения в Rust",
  location: [#github-link("nekitdev/ownership")],
  date: "Август 2025 - Настоящее время",
  description: "Дизайнер/Разработчик",
)

#resume-item[
  - Дизайн и реализация производных макросов для получения владения в Rust.
  - Написание документации и тестов, публикация библиотеки на crates.io.
]

#resume-entry(
  title: "Графовые структуры данных и алгоритмы",
  location: [#github-link("nekitdev/graphs")],
  date: "Июнь 2025 - Настоящее время",
  description: "Дизайнер/Разработчик",
)

#resume-item[
  - Дизайн и реализация графовых структур данных и алгоритмов в Rust.
  - Написание документации и тестов, публикация библиотеки на GitHub.
]

#resume-entry(
  title: "Типы уточнения",
  location: [#github-link("nekitdev/refining")],
  date: "Февраль 2025 - Настоящее время",
  description: "Дизайнер/Разработчик",
)

#resume-item[
  - Дизайн и реализация типов уточнения для Rust.
  - Написание документации и тестов, публикация библиотеки на crates.io.
]

#resume-entry(
  title: "Личный сайт",
  location: [#github-link("nekitdev/nekit.dev")],
  date: "Сентябрь 2024 - Настоящее время",
  description: "Дизайнер/Разработчик",
)

#resume-item[
  - Дизайн и разработка личного сайта с использованием Rust и Dioxus.
  - Реализация адаптивного дизайна и оптимизация производительности.
]

#resume-entry(
  title: "Сборка истории изменений из фрагментов",
  location: [#github-link("nekitdev/changelogging")],
  date: "Май 2024 - Настоящее время",
  description: "Дизайнер/Разработчик",
)

#resume-item[
  - Дизайн и разработка CLI-инструмента для сборки истории изменений из фрагментов на Rust.
  - Написание документации и тестов, публикация инструмента на crates.io.
]

= Навыки

#resume-skill-grid(
  categories-with-values: (
    "Языки программирования": (
      strong("Rust"),
      strong("Python"),
      "TypeScript",
      "SQL",
      "C#",
    ),
    "Разговорные языки": (
      strong("Русский"),
      strong("Английский"),
    ),
  ),
)

= Образование

#resume-entry(
  date: "2024 - Настоящее время",
  location: "МИРЭА - Российский технологический университет",
  title: "Информационные системы и технологии",
  description: "Фуллстек разработка",
)
