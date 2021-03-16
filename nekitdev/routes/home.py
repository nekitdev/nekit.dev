from datetime import date

from nekitdev.core import env, html_response, routes, web
from nekitdev.projects import PROJECTS

template = env.get_template("home.html")

birthday = date(2005, 1, 13)  # 13th January, 2005
year = 365


@routes.get("/")
async def home(request: web.Request) -> web.Response:
    age = (date.today() - birthday).days // year

    return html_response(text=await template.render_async(age=age, projects=PROJECTS))
