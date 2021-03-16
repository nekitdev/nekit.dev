from yarl import URL

__all__ = ("PROJECTS", "Project")

BASE = URL("https://github.com/")


class Project:
    def __init__(self, author: str, name: str, description: str) -> None:
        self._author = author
        self._name = name
        self._description = description

    @property
    def author(self) -> str:
        return self._author

    @property
    def name(self) -> str:
        return self._name

    @property
    def description(self) -> str:
        return self._description

    @property
    def url(self) -> URL:
        return BASE / self.author / self.name

    @property
    def link(self) -> str:
        return self.url.human_repr()


PROJECTS = (
    Project("nekitdev", "gd.py", "An API Wrapper for Geometry Dash written in Python."),
    Project("nekitdev", "iters.py", "Rich Iterators for Python."),
    Project("nekitdev", "enums.py", "Enhanced Enum Implementation for Python."),
    Project(
        "nekitdev", "braces.py", "Implementation of Braces (Curly Brackets) Syntax for Python."
    ),
    Project(
        "nekitdev", "bulma.py", "Small Compiler for Bulma and Extensions."
    ),
    Project(
        "nekitdev", "nekit.dev", "Web Application written in Python with Bulma."
    ),
)
