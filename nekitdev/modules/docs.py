from aiohttp.web import HTTPFound, HTTPNotFound, Request, Response
from yarl import URL

from nekitdev.constants import DOCS_LINK
from nekitdev.core import routes
from nekitdev.utils import check_back, identifier

NAME = "name"

DOCS_ROUTE = f"/docs/{identifier(NAME)}"

DOCS = URL(DOCS_LINK)


@routes.get(DOCS_ROUTE)
def handle_docs(request: Request) -> Response:
    name = request.match_info[NAME]

    if check_back(name):
        raise HTTPNotFound()

    raise HTTPFound(DOCS / name)
