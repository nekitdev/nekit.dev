from typing import Awaitable, Callable, TypeVar

from aiohttp.web import Request, StreamResponse

__all__ = ("Unary", "Binary", "Handler", "Middleware")

T = TypeVar("T")
U = TypeVar("U")
R = TypeVar("R")

Unary = Callable[[T], R]
Binary = Callable[[T, U], R]

Handler = Unary[Request, Awaitable[StreamResponse]]
Middleware = Binary[Request, Handler, StreamResponse]
