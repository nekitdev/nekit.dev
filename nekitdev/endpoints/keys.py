from fastapi.responses import PlainTextResponse

from nekitdev.core import app
from nekitdev.errors import NotFound
from nekitdev.constants import DEFAULT_ENCODING, DEFAULT_ERRORS, KEY_SUFFIX, KEYS

__all__ = ("get_key",)

CAN_NOT_FIND_KEY = "can not find `{}` key"


@app.get("/keys/{name}")
async def get_key(name: str) -> PlainTextResponse:
    path = (KEYS / name).with_suffix(KEY_SUFFIX)

    if path.exists():
        return PlainTextResponse(path.read_text(DEFAULT_ENCODING, DEFAULT_ERRORS))

    raise NotFound(CAN_NOT_FIND_KEY.format(name))
