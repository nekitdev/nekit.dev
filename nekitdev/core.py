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

case_fold = str.casefold


def register_error_handlers(app: FastAPI) -> None:
    @app.exception_handler(Error)
    async def error_handler(request: Request, error: Error) -> HTMLResponse:
        return HTMLResponse(
            await ERROR_TEMPLATE.render_async(code=error.code.value, message=error.message),
            status_code=error.status_code,
        )

    def validation_error_to_message(validation_error: RequestValidationError) -> str:
        return str(validation_error).replace(NEW_LINE, BREAK)

    @app.exception_handler(RequestValidationError)
    async def validation_error_handler(
        request: Request, error: RequestValidationError
    ) -> HTMLResponse:
        converted_error = ValidationError(validation_error_to_message(error))

        return await error_handler(request, converted_error)

    @app.exception_handler(HTTPError)
    async def http_error_handler(request: Request, error: HTTPError) -> HTMLResponse:
        message = case_fold(error.detail)

        status_code = error.status_code

        code = ErrorCode.from_status_code(status_code)

        converted_error = Error(message, code, status_code)

        return await error_handler(request, converted_error)

    @app.exception_handler(NormalError)
    async def internal_error_handler(request: Request, error: NormalError) -> HTMLResponse:
        internal_error = InternalError()

        return await error_handler(request, internal_error)


register_error_handlers(app)
