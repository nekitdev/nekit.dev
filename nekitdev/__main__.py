import click

import gd
import nekitdev

HOST = "0.0.0.0"
PORT = 8080


@click.option("--host", type=str, default=HOST, help="Host to serve the application on.")
@click.option("--port", type=int, default=PORT, help="Port to serve the application on.")
@click.command(name="nekitdev", short_help="Run the web application.")
def main(host: str, port: int) -> None:
    app = gd.server.create_app()

    gd.server.setup_gd_app_sync(app)

    nekitdev.setup_app(app)

    gd.server.run_app_sync(app, host=host, port=port)


if __name__ == "__main__":
    main()
