from yarl import URL
from nekitdev.core import routes, web

DOCS = URL("https://nekitdev.github.io/")

IDENTIFIER = r"[A-Za-z0-9_\.\-]+"
NAME = "name"


@routes.get(f"/docs/{{{NAME}:{IDENTIFIER}}}")
async def handle_github(request: web.Request) -> web.Response:
    name = request.match_info[NAME]

    raise web.HTTPFound(DOCS / name)
