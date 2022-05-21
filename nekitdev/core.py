import os
from pathlib import Path
from typing import Any

import bulma
import gd

from aiohttp import web
from jinja2 import Environment, FileSystemLoader

__all__ = (
    "html_response",
    "env",
    "root",
    "setup_app",
    "web",
)

root = Path(__file__).parent

static = "/static"
static_path = root / "static"
templates = root / "templates"

compiler = bulma.Compiler(
    # extensions to use
    extensions=[],
    # variables
    variables={
        # set default font to monospace
        "family-monospace": "menlo, consolas, monospace",
        "family-primary": "$family-monospace",
        # set primary color to something of our choice
        "primary": "$purple",
    },
    # dark theme simple setup
    themes=["dark"],
    dark_variables={
        "scheme-main": "$black",
        "scheme-invert": "$white",
    },
    # compress the output
    output_style=bulma.COMPRESSED,
    # custom files
    custom=[],
)

include = compiler.save(static_path).with_static(static)

env = Environment(
    loader=FileSystemLoader(templates),
    trim_blocks=True,
    lstrip_blocks=True,
    enable_async=True,
)

env.globals.update(include=include, gd=gd)

routes = web.RouteTableDef()

routes.static(static, static_path)

HTML = "text/html"


def html_response(*args: Any, **kwargs: Any) -> web.Response:
    kwargs.update(content_type=HTML)

    return web.Response(*args, **kwargs)


def setup_app(app: web.Application) -> web.Application:
    app.add_routes(routes)

    return app
