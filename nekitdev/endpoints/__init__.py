from nekitdev.endpoints.docs import redirect_docs
from nekitdev.endpoints.email import redirect_email
from nekitdev.endpoints.github import redirect_github
from nekitdev.endpoints.index import get_index
from nekitdev.endpoints.keys import get_key
from nekitdev.endpoints.redirect import create_redirect

__all__ = (
    # docs
    "redirect_docs",
    # email
    "redirect_email",
    # github
    "redirect_github",
    # index
    "get_index",
    # keys
    "get_key",
    # redirect
    "create_redirect",
)
