"""German salutations for Switzerland, Liechtenstein, Germany, and Austria."""

import re
from typing import Literal

from ._tables import TABLES

Region = Literal["ch", "li", "de", "at"]
_DE = TABLES["de"]
_WS = re.compile(r"[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]+")
_ASCII_LOWER = str.maketrans("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz")

__all__ = ["parse_region", "region_uses_comma", "salutation_last_name",
           "salutation_honorific", "salutation_titles", "salutation_surname",
           "de_salutation", "recipient_salutation_warning", "de_honorific_warning"]


def _tokens(name: str) -> list[str]:
    return [token for token in _WS.split(name) if token]


def _norm(token: str) -> str:
    return token.strip(".").translate(_ASCII_LOWER)


def parse_region(code: str) -> Region | None:
    """Accept only the four lowercase region codes."""
    return code if code in ("ch", "li", "de", "at") else None


def region_uses_comma(region: Region) -> bool:
    """Whether a German salutation ends with a comma."""
    return region in _DE["comma_regions"]


def salutation_last_name(name: str) -> str:
    """Last whitespace-separated token, or an empty string."""
    tokens = _tokens(name)
    return tokens[-1] if tokens else ""


def salutation_honorific(name: str) -> str:
    """Explicit Herr/Herrn or Frau; never infer an honorific from a name."""
    tokens = _tokens(name)
    return _DE["honorifics"].get(_norm(tokens[0]), "") if tokens else ""


def salutation_titles(name: str) -> list[str]:
    """Keep recognized academic titles, with Professor taking precedence."""
    kept = []
    for token in _tokens(name):
        title = _DE["titles"].get(_norm(token))
        if title is not None and title not in kept:
            kept.append(title)
    for title in kept:
        if title in _DE["sole_titles"]:
            return [title]
    return kept


def salutation_surname(name: str) -> str:
    """Last token after excluding honorifics, titles, and qualifications."""
    return next((token for token in reversed(_tokens(name)) if _norm(token) not in _DE["filler"]), "")


def de_salutation(name: str, region: Region) -> str:
    """Format a German salutation, using the generic address when incomplete."""
    if parse_region(region) is None:
        raise ValueError("region must be ch, li, de, or at")
    punct = "," if region_uses_comma(region) else ""
    honorific, surname = salutation_honorific(name), salutation_surname(name)
    if not honorific or not surname:
        return _DE["generic"] + punct
    titles = salutation_titles(name)
    title_part = " " + " ".join(titles) if titles else ""
    address = "Sehr geehrte Frau" if honorific == "frau" else "Sehr geehrter Herr"
    return f"{address}{title_part} {surname}{punct}"


def recipient_salutation_warning(location: str, name: str) -> str | None:
    """Advisory for an empty recipient name."""
    if salutation_last_name(name) == "":
        return f"{location}: job.cl_recipient.name is empty; using generic salutation (provide a name for tailored opportunities)"
    return None


def de_honorific_warning(location: str, name: str) -> str | None:
    """Advisory for a nonempty German name without a usable honorific."""
    if salutation_last_name(name) and (not salutation_honorific(name) or not salutation_surname(name)):
        return f'{location}: job.cl_recipient.name has no parsable Herr/Frau honorific; using generic salutation (provide e.g. "Frau Dr. Müller" for tailored opportunities)'
    return None
