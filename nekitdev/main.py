from pathlib import Path
import subprocess
from sys import exit
from typing import Iterable

import click
import gd

from nekitdev.constants import (
    DEFAULT_HOST, DEFAULT_INPUT, DEFAULT_NAME, DEFAULT_OUTPUT, DEFAULT_PORT, ROOT
)
from nekitdev.core import setup_app

EXECUTE = "npx"
TAILWIND = "tailwindcss"
INPUT = "-i"
OUTPUT = "-o"
MINIFY = "--minify"
WATCH = "--watch"


def build_command(input: Path, output: Path, watch: bool) -> Iterable[str]:
    arguments = [EXECUTE, TAILWIND, INPUT, input.as_posix(), OUTPUT, output.as_posix(), MINIFY]

    if watch:
        arguments.append(WATCH)

    return arguments


@click.group(name=DEFAULT_NAME)
def nekitdev() -> None:
    pass


@click.option("--host", "-h", type=str, default=DEFAULT_HOST)
@click.option("--port", "-p", type=int, default=DEFAULT_PORT)
@nekitdev.command()
def run(host: str, port: int) -> None:
    create_and_run(host, port)


def create_and_run(host: str, port: int) -> None:
    app = gd.server.create_app()

    gd.server.setup_gd_app_sync(app)

    setup_app(app)

    gd.server.run_app_sync(app, host=host, port=port)


@click.option("--input", "-i", type=Path, default=DEFAULT_INPUT)
@click.option("--output", "-o", type=Path, default=DEFAULT_OUTPUT)
@click.option("--watch", "-w", is_flag=True, default=False)
@nekitdev.command()
def build(input: Path, output: Path, watch: bool) -> None:
    exit(
        subprocess.call(build_command(input, output, watch=watch), cwd=ROOT.as_posix(), shell=True)
    )
