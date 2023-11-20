from pathlib import Path
from subprocess import call
from sys import exit
from typing import Sequence

import click
import uvicorn

from nekitdev.constants import (
    DEFAULT_HOST,
    DEFAULT_INPUT,
    DEFAULT_OUTPUT,
    DEFAULT_PORT,
    DEFAULT_WATCH,
    ROOT,
)
from nekitdev.core import app

EXECUTE = "npx"
TAILWIND = "tailwindcss"
INPUT = "-i"
OUTPUT = "-o"
MINIFY = "-m"
WATCH = "-w"


def build_command(input: Path, output: Path, watch: bool) -> Sequence[str]:
    arguments = [EXECUTE, TAILWIND, INPUT, input.as_posix(), OUTPUT, output.as_posix(), MINIFY]

    if watch:
        arguments.append(WATCH)

    return arguments


@click.group()
def nekitdev() -> None:
    pass


@click.option("--host", "-h", type=str, default=DEFAULT_HOST)
@click.option("--port", "-p", type=int, default=DEFAULT_PORT)
@nekitdev.command()
def run(host: str, port: int) -> None:
    uvicorn.run(app, host=host, port=port)


SHELL = False


@click.option("--input", "-i", type=Path, default=DEFAULT_INPUT)
@click.option("--output", "-o", type=Path, default=DEFAULT_OUTPUT)
@click.option("--watch", "-w", is_flag=True, default=DEFAULT_WATCH)
@nekitdev.command()
def build(input: Path, output: Path, watch: bool) -> None:
    exit(call(build_command(input, output, watch=watch), cwd=ROOT, shell=SHELL))
