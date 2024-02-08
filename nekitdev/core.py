from fastapi.applications import FastAPI
from fastapi.exceptions import RequestValidationError
from fastapi.requests import Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from jinja2 import Environment, FileSystemLoader
from starlette.exceptions import HTTPException as HTTPError
from typing_aliases import NormalError

from nekitdev.constants import BREAK, NEW_LINE, STATIC, STATIC_NAME, STATIC_PATH, TEMPLATES
from nekitdev.errors import Error, ErrorCode, InternalError, ValidationError

__all__ = ("app", "environment")

environment = Environment(
    loader=FileSystemLoader(TEMPLATES),
    trim_blocks=True,
    lstrip_blocks=True,
    autoescape=True,
    enable_async=True,
)

ERROR_TEMPLATE = environment.get_template("error.html")

app = FastAPI()

app.mount(STATIC_PATH, StaticFiles(directory=STATIC), name=STATIC_NAME)

UNHANDLED_ERROR = "unhandled error"


def register_error_handlers(app: FastAPI) -> None:
    @app.exception_handler(Error)
    async def error_handler(request: Request, error: Error) -> HTMLResponse:
        return HTMLResponse(
            await ERROR_TEMPLATE.render_async(message=error.message, code=error.CODE.value),
            status_code=error.STATUS_CODE,
        )

    def error_to_message(error: NormalError) -> str:
        return str(error).replace(NEW_LINE, BREAK)

    @app.exception_handler(RequestValidationError)
    async def validation_error_handler(
        request: Request, error: RequestValidationError
    ) -> HTMLResponse:
        converted_error = ValidationError(error_to_message(error))

        return await error_handler(request, converted_error)

    @app.exception_handler(HTTPError)
    async def http_error_handler(request: Request, error: HTTPError) -> HTMLResponse:
        message = error_to_message(error)
        status_code = error.status_code
        code = ErrorCode.from_status_code(status_code)

        return HTMLResponse(
            await ERROR_TEMPLATE.render_async(message=message, code=code), status_code=status_code
        )

    @app.exception_handler(NormalError)
    async def internal_error_handler(request: Request, error: NormalError) -> HTMLResponse:
        internal_error = InternalError(UNHANDLED_ERROR)

        return await error_handler(request, internal_error)


register_error_handlers(app)
