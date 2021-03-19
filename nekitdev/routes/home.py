from datetime import date

from nekitdev.core import env, html_response, routes, web

template = env.get_template("home.html")

nekit_birthday = date(2005, 1, 13)  # 13th January, 2005
year = 365


def get_age(birthday: date = nekit_birthday) -> int:
    return (date.today() - birthday).days // year


@routes.get("/")
async def home(request: web.Request) -> web.Response:
    return html_response(text=await template.render_async(age=get_age()))
