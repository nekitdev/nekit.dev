from fastapi.responses import HTMLResponse

from nekitdev.core import app, environment
from nekitdev.utils import get_nekit_age

__all__ = ("get_index",)

INDEX_TEMPLATE = environment.get_template("index.html")


@app.get("/")
async def get_index() -> HTMLResponse:
    return HTMLResponse(await INDEX_TEMPLATE.render_async(age=get_nekit_age()))
