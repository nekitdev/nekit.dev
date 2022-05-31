from yarl import URL
from nekitdev.core import routes, web

GITHUB = URL("https://github.com/nekitdev")

IDENTIFIER = r"[A-Za-z0-9_\.\-]+"
NAME = "name"


@routes.get(f"/github/{{{NAME}:{IDENTIFIER}}}")
async def handle_github(request: web.Request) -> web.Response:
    name = request.match_info[NAME]

    raise web.HTTPFound(GITHUB / name)
