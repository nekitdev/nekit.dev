import click
import uvicorn

from nekitdev.constants import DEFAULT_HOST, DEFAULT_PORT
from nekitdev.core import app


@click.option("--host", "-h", type=str, default=DEFAULT_HOST)
@click.option("--port", "-p", type=int, default=DEFAULT_PORT)
@click.command()
def nekitdev(host: str, port: int) -> None:
    uvicorn.run(app, host=host, port=port)
