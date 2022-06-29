from pathlib import Path

ROOT = Path(__file__).parent

ROOT_ROUTE = "/"

TEXT_HTML = "text/html"
TEXT_PLAIN = "text/plain"

BASE_NAME = "base.html"
HOME_NAME = "home.html"

CSS_NAME = "css"
KEYS_NAME = "keys"
STATIC_NAME = "static"
TEMPLATES_NAME = "templates"

KEY_SUFFIX = ".key"

KEYS = ROOT / KEYS_NAME
STATIC = ROOT / STATIC_NAME
CSS = STATIC / CSS_NAME
TEMPLATES = ROOT / TEMPLATES_NAME

DEFAULT_INPUT_NAME = "input.css"
DEFAULT_OUTPUT_NAME = "output.css"

DEFAULT_INPUT = CSS / DEFAULT_INPUT_NAME
DEFAULT_OUTPUT = CSS / DEFAULT_OUTPUT_NAME

DOCS_LINK = "https://nekitdev.github.io/"

DISCORD_NAME = "discord"

DISCORD_LINK = "https://discord.com/invite/KtJkbut"

GITHUB_NAME = "github"
KEYBASE_NAME = "keybase"
PATREON_NAME = "patreon"
TWITTER_NAME = "twitter"
YOUTUBE_NAME = "youtube"

GITHUB_LINK = "https://github.com/nekitdev"
KEYBASE_LINK = "https://keybase.io/nekitdev"
PATREON_LINK = "https://patreon.com/nekitdev"
TWITTER_LINK = "https://twitter.com/nekitdev"
YOUTUBE_LINK = "https://youtube.com/nekitdev"

NAME_TO_LINK = {
    DISCORD_NAME: DISCORD_LINK,
    GITHUB_NAME: GITHUB_LINK,
    KEYBASE_NAME: KEYBASE_LINK,
    PATREON_NAME: PATREON_LINK,
    TWITTER_NAME: TWITTER_LINK,
    YOUTUBE_NAME: YOUTUBE_LINK,
}

DEFAULT_HOST = "0.0.0.0"
DEFAULT_PORT = 1342

DEFAULT_NAME = "nekitdev"
