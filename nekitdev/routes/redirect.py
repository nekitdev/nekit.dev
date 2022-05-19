from nekitdev.core import routes, web

DISCORD = "discord"

DISCORD_LINK = "https://discord.gg/KtJkbut"

GITHUB = "github"
KEYBASE = "keybase"
PATREON = "patreon"
TWITTER = "twitter"
YOUTUBE = "youtube"

GITHUB_LINK = "https://github.com/nekitdev"
KEYBASE_LINK = "https://keybase.io/nekitdev"
PATREON_LINK = "https://patreon.com/nekitdev"
TWITTER_LINK = "https://twitter.com/nekitdev"
YOUTUBE_LINK = "https://youtube.com/nekitdev"

NAME_TO_LINK = {
    DISCORD: DISCORD_LINK,
    GITHUB: GITHUB_LINK,
    KEYBASE: KEYBASE_LINK,
    PATREON: PATREON_LINK,
    TWITTER: TWITTER_LINK,
    YOUTUBE: YOUTUBE_LINK,
}


def create_redirect(name: str, link: str) -> None:
    @routes.get(f"/{name}")
    async def handle_redirect(request: web.Request) -> web.Response:
        raise web.HTTPFound(link)


for name, link in NAME_TO_LINK.items():
    create_redirect(name, link)
