from nekitdev.core import root, routes, web

IDENTIFIER = r"[A-Za-z0-9_\.\-]+"

NAME = "name"
KEY = ".key"
KEYS = "keys"

keys = root / KEYS


@routes.get(f"/{KEYS}/{{{NAME}:{IDENTIFIER}}}")
async def handle_keys(request: web.Request) -> web.Response:
    name = request.match_info[NAME]

    path = (keys / name).with_suffix(KEY)

    if path.exists():
        return web.Response(text=path.read_text())

    raise web.HTTPNotFound()
