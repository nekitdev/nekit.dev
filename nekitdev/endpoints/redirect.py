from fastapi import status
from fastapi.responses import RedirectResponse

from nekitdev.core import app

ROOT = "/"

CHAT_NAME = "chat"

CHAT_LINK = "https://discord.com/invite/KtJkbut"

DISCORD_NAME = "discord"

DISCORD_LINK = "https://discord.com/users/292448864944783360"

GITHUB_NAME = "github"
KEYBASE_NAME = "keybase"
X_NAME = "x"
YOUTUBE_NAME = "youtube"
REDDIT_NAME = "reddit"
TELEGRAM_NAME = "telegram"
FUNDING_NAME = "funding"

GITHUB_LINK = "https://github.com/nekitdev"
KEYBASE_LINK = "https://keybase.io/nekitdev"
X_LINK = "https://x.com/nekitdev"
YOUTUBE_LINK = "https://youtube.com/nekitdev"
REDDIT_LINK = "https://reddit.com/u/nekitdev"
TELEGRAM_LINK = "https://t.me/nekitdev"
FUNDING_LINK = "https://boosty.to/nekitdev"

NAME_TO_LINK = {
    CHAT_NAME: CHAT_LINK,
    DISCORD_NAME: DISCORD_LINK,
    GITHUB_NAME: GITHUB_LINK,
    KEYBASE_NAME: KEYBASE_LINK,
    X_NAME: X_LINK,
    YOUTUBE_NAME: YOUTUBE_LINK,
    REDDIT_NAME: REDDIT_LINK,
    TELEGRAM_NAME: TELEGRAM_LINK,
    FUNDING_NAME: FUNDING_LINK,
}


def create_redirect(name: str, link: str) -> None:
    @app.get(ROOT + name)
    async def _redirect() -> RedirectResponse:
        return RedirectResponse(link, status_code=status.HTTP_302_FOUND)


for name, link in NAME_TO_LINK.items():
    create_redirect(name, link)
