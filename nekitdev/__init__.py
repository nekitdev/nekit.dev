"""nekit.dev - Personal website written in Python."""

__description__ = "nekit.dev - Personal website written in Python."
__url__ = "https://github.com/nekitdev/nekit.dev"

__title__ = "nekitdev"
__author__ = "nekitdev"
__license__ = "MIT"
__version__ = "1.0.0-alpha.1"

from nekitdev import modules
from nekitdev.core import setup_app
from nekitdev.main import create_and_run_app

__all__ = ("modules", "create_and_run_app", "setup_app")
