from nekitdev.core import routes, web

DISCORD = "discord"

DISCORD_LINK = "https://discord.gg/KtJkbut"

GITHUB = "github"
PATREON = "patreon"
TWITTER = "twitter"
YOUTUBE = "youtube"

GITHUB_LINK = "https://github.com/nekitdev"
PATREON_LINK = "https://patreon.com/nekitdev"
TWITTER_LINK = "https://twitter.com/nekitdev"
YOUTUBE_LINK = "https://youtube.com/nekitdev"

NAME_TO_LINK = {
    DISCORD: DISCORD_LINK,
    GITHUB: GITHUB_LINK,
    PATREON: PATREON_LINK,
    TWITTER: TWITTER_LINK,
    YOUTUBE: YOUTUBE_LINK,
}

NAME = "name"


@routes.get("/{name}")
async def handle_possible_redirect(request: web.Request) -> web.Response:
    name = request.match_info[NAME]

    link = NAME_TO_LINK.get(name)

    if link is None:
        raise web.HTTPNotFound()

    raise web.HTTPFound(link)
