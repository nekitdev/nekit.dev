from fastapi import status
from fastapi.responses import RedirectResponse

from nekitdev.core import app

__all__ = ("redirect_github",)

GITHUB = "https://github.com/nekitdev/"


@app.get("/github/{name}")
async def redirect_github(name: str) -> RedirectResponse:
    return RedirectResponse(GITHUB + name, status_code=status.HTTP_302_FOUND)
