from fastapi import status
from fastapi.responses import RedirectResponse

from nekitdev.core import app

__all__ = ("redirect_docs",)

DOCS = "https://nekitdev.github.io/"


@app.get("/docs/{name}")
async def redirect_docs(name: str) -> RedirectResponse:
    return RedirectResponse(DOCS + name, status_code=status.HTTP_302_FOUND)
