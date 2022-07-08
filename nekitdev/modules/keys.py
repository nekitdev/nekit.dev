from aiohttp.web import HTTPNotFound, Request, Response

from nekitdev.constants import KEY_SUFFIX, KEYS, TEXT_PLAIN
from nekitdev.core import routes
from nekitdev.utils import check_back, identifier

NAME = "name"

KEYS_ROUTE = f"/keys/{identifier(NAME)}"


@routes.get(KEYS_ROUTE)
async def handle_keys(request: Request) -> Response:
    name = request.match_info[NAME]

    if check_back(name):
        raise HTTPNotFound()

    path = (KEYS / name).with_suffix(KEY_SUFFIX)

    if path.exists():
        return Response(content_type=TEXT_PLAIN, text=path.read_text())

    raise HTTPNotFound()
