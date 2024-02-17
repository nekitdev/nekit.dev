from __future__ import annotations

from enum import Enum
from typing import ClassVar, Optional, Type, TypeVar, final

from attrs import frozen
from fastapi import status
from typing_aliases import NormalError
from typing_extensions import TypedDict as Data

__all__ = ("Error", "ErrorCode", "ErrorData", "ErrorType", "ValidationError", "InternalError")


class ErrorCode(Enum):
    DEFAULT = 13000

    BAD_REQUEST = 13400
    UNAUTHORIZED = 13401
    FORBIDDEN = 13403
    NOT_FOUND = 13404
    METHOD_NOT_ALLOWED = 13405
    CONFLICT = 13409
    GONE = 13410
    PAYLOAD_TOO_LARGE = 13413
    UNPROCESSABLE_ENTITY = 13422
    TOO_MANY_REQUESTS = 13429

    INTERNAL_SERVER_ERROR = 13500

    @classmethod
    def from_status_code(cls, status_code: int) -> ErrorCode:
        default = cls.DEFAULT

        try:
            return cls(default.value + status_code)

        except ValueError:
            return default


class ErrorData(Data):
    code: int
    message: str


ERROR = "error"


class Error(NormalError):
    DEFAULT_CODE: ClassVar[ErrorCode]
    DEFAULT_STATUS_CODE: ClassVar[int]

    def __init__(
        self,
        message: str,
        code: Optional[ErrorCode] = None,
        status_code: Optional[int] = None,
    ) -> None:
        super().__init__(message)

        if code is None:
            code = self.DEFAULT_CODE

        if status_code is None:
            status_code = self.DEFAULT_STATUS_CODE

        self._message = message
        self._code = code
        self._status_code = status_code

    @property
    def message(self) -> str:
        return self._message

    @property
    def code(self) -> ErrorCode:
        return self._code

    @property
    def status_code(self) -> int:
        return self._status_code

    def into_data(self) -> ErrorData:
        return ErrorData(code=self.code.value, message=self.message)


ErrorType = Type[Error]


ET = TypeVar("ET", bound=ErrorType)


@final
@frozen()
class DefaultCode:
    code: ErrorCode

    def __call__(self, error_type: ET) -> ET:
        error_type.DEFAULT_CODE = self.code

        return error_type


def default_code(code: ErrorCode) -> DefaultCode:
    return DefaultCode(code)


@final
@frozen()
class DefaultStatusCode:
    status_code: int

    def __call__(self, error_type: ET) -> ET:
        error_type.DEFAULT_STATUS_CODE = self.status_code

        return error_type


def default_status_code(status_code: int) -> DefaultStatusCode:
    return DefaultStatusCode(status_code)


@default_code(ErrorCode.UNPROCESSABLE_ENTITY)
@default_status_code(status.HTTP_422_UNPROCESSABLE_ENTITY)
class ValidationError(Error):
    """Validation has failed."""


INTERNAL_ERROR = "internal error"


@default_code(ErrorCode.INTERNAL_SERVER_ERROR)
@default_status_code(status.HTTP_500_INTERNAL_SERVER_ERROR)
class InternalError(Error):
    """Internal error has occured."""

    def __init__(self) -> None:
        super().__init__(INTERNAL_ERROR)
