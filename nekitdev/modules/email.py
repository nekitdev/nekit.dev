from aiohttp.web import HTTPFound, Request, Response

from nekitdev.constants import DOMAIN, EMAIL, EMAIL_TO
from nekitdev.core import routes
from nekitdev.utils import identifier

NAME = "name"
EMAIL_ROUTE = f"/email/{identifier(NAME)}"


@routes.get(EMAIL_ROUTE)
async def handle_email(request: Request) -> Response:
    name = request.match_info[NAME]

    raise HTTPFound(EMAIL_TO + EMAIL.format(name, DOMAIN))
