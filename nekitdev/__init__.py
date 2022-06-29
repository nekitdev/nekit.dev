"""nekit.dev - Web application written in Python."""

__description__ = "nekit.dev - Web application written in Python."
__url__ = "https://github.com/nekitdev/nekit.dev"

__title__ = "nekitdev"
__author__ = "nekitdev"
__license__ = "MIT"
__version__ = "1.0.0-alpha.1"

from nekitdev import modules
from nekitdev.core import setup_app

__all__ = ("modules", "setup_app")
