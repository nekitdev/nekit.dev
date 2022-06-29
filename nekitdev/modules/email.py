from aiohttp.web import HTTPFound, Request, Response

from nekitdev.constants import EMAIL_TO, EMAIL, DOMAIN
from nekitdev.core import routes
from nekitdev.utils import identifier

NAME = "name"
EMAIL_ROUTE = f"/email/{identifier(NAME)}"


@routes.get(EMAIL_ROUTE)
def handle_docs(request: Request) -> Response:
    name = request.match_info[NAME]

    raise HTTPFound(EMAIL_TO + EMAIL.format(name, DOMAIN))
