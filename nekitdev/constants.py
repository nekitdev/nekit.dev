from pathlib import Path

from pendulum import Date

# constants

ROOT = Path(__file__).parent

NEW_LINE = "\n"
BREAK = "<br>"

STATIC_PATH = "/static"

CSS_NAME = "css"
KEYS_NAME = "keys"
STATIC_NAME = "static"
TEMPLATES_NAME = "templates"

KEY_SUFFIX = ".key"

KEYS = ROOT / KEYS_NAME
STATIC = ROOT / STATIC_NAME
TEMPLATES = ROOT / TEMPLATES_NAME

# defaults

DEFAULT_ENCODING = "utf-8"
DEFAULT_ERRORS = "strict"

DEFAULT_HOST = "127.0.0.1"
DEFAULT_PORT = 6942

# birthday

NEKIT_BIRTHDAY = Date(2005, 1, 13)
