from aiohttp.web import HTTPFound, HTTPNotFound, Request, Response
from yarl import URL

from nekitdev.core import routes
from nekitdev.utils import check_back, identifier

GITHUB = URL("https://github.com/nekitdev")

NAME = "name"

GITHUB_ROUTE = f"/github/{identifier(NAME)}"


@routes.get(GITHUB_ROUTE)
async def handle_github(request: Request) -> Response:
    name = request.match_info[NAME]

    if check_back(name):
        raise HTTPNotFound()

    raise HTTPFound(GITHUB / name)
