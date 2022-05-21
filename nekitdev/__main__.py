import click
import gd
from entrypoint import entrypoint

import nekitdev

NEKITDEV = "nekitdev"

HOST = "0.0.0.0"
PORT = 8080


@entrypoint(__name__)
@click.option("--host", type=str, default=HOST, help="Host to serve the application on.")
@click.option("--port", type=int, default=PORT, help="Port to serve the application on.")
@click.command(name=NEKITDEV, short_help="Run the web application.")
def main(host: str, port: int) -> None:
    app = gd.server.create_app()

    gd.server.setup_gd_app_sync(app)

    nekitdev.setup_app(app)

    gd.server.run_app_sync(app, host=host, port=port)
