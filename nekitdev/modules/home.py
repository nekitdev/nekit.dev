from datetime import date

from aiohttp.web import Request, Response
from nekitdev.constants import HOME_NAME, ROOT_ROUTE, TEXT_HTML

from nekitdev.core import environment, routes

template = environment.get_template(HOME_NAME)

nekit_birth = date(2005, 1, 13)  # 13th January, 2005


def get_age(current: date, birth: date) -> int:
    age = current.year - birth.year

    if (current.month, current.day) < (birth.month, birth.day):
        age -= 1

    return age


def get_nekit_age() -> int:
    return get_age(date.today(), nekit_birth)


@routes.get(ROOT_ROUTE)
async def home(request: Request) -> Response:
    return Response(
        content_type=TEXT_HTML, text=await template.render_async(age=get_nekit_age())
    )
