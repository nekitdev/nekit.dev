from pendulum import UTC, Date, DateTime, now

from nekitdev.constants import NEKIT_BIRTHDAY

__all__ = ("utc_now", "utc_today", "get_age", "get_nekit_age")


def utc_now() -> DateTime:
    return now(UTC)


def utc_today() -> Date:
    return utc_now().date()


def get_age(birthday: Date) -> int:
    return utc_today().diff(birthday).in_years()


def get_nekit_age() -> int:
    return get_age(NEKIT_BIRTHDAY)
