"""Personal website written in Python."""

__description__ = "Personal website written in Python."
__url__ = "https://github.com/nekitdev/nekit.dev"

__title__ = "nekitdev"
__author__ = "nekitdev"
__license__ = "MIT"
__version__ = "1.0.0"

from nekitdev import endpoints
from nekitdev.core import app, environment

__all__ = ("endpoints", "app", "environment")
