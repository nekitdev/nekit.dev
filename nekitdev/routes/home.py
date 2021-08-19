from datetime import date

from nekitdev.core import env, html_response, routes, web

template = env.get_template("home.html")

nekit_birth = date(2005, 1, 13)  # 13th January, 2005


def get_age(current: date, birth: date) -> int:
    age = current.year - birth.year

    if (current.month, current.day) < (birth.month, birth.day):
        age -= 1

    return age


def get_nekit_age() -> int:
    return get_age(date.today(), nekit_birth)


@routes.get("/")
async def home(request: web.Request) -> web.Response:
    return html_response(text=await template.render_async(age=get_nekit_age()))
