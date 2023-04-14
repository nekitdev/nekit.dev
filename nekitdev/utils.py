from pendulum import UTC, Date, DateTime, now

from nekitdev.constants import NEKIT_BIRTHDAY

__all__ = ("utc_now", "utc_today", "get_age", "get_nekit_age")


def utc_now() -> DateTime:
    return now(UTC)


def utc_today() -> Date:
    return utc_now().date()  # type: ignore


def get_age(birthday: Date) -> int:
    return utc_today().diff(birthday).in_years()  # type: ignore


def get_nekit_age() -> int:
    return get_age(NEKIT_BIRTHDAY)
