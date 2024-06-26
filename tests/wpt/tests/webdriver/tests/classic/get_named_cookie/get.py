import pytest

from datetime import datetime, timedelta, timezone


from tests.support.asserts import assert_error, assert_success
from tests.support.helpers import clear_all_cookies


def get_named_cookie(session, name):
    return session.transport.send(
        "GET", "session/{session_id}/cookie/{name}".format(
            session_id=session.session_id,
            name=name))


def test_no_top_browsing_context(session, closed_window):
    response = get_named_cookie(session, "foo")
    assert_error(response, "no such window")


def test_no_browsing_context(session, closed_frame):
    response = get_named_cookie(session, "foo")
    assert_error(response, "no such window")


def test_get_named_session_cookie(session, url):
    session.url = url("/common/blank.html")
    clear_all_cookies(session)
    session.execute_script("document.cookie = 'foo=bar'")

    result = get_named_cookie(session, "foo")
    cookie = assert_success(result)
    assert isinstance(cookie, dict)

    # table for cookie conversion
    # https://w3c.github.io/webdriver/#dfn-table-for-cookie-conversion
    assert "name" in cookie
    assert isinstance(cookie["name"], str)
    assert "value" in cookie
    assert isinstance(cookie["value"], str)
    assert "path" in cookie
    assert isinstance(cookie["path"], str)
    assert "domain" in cookie
    assert isinstance(cookie["domain"], str)
    assert "secure" in cookie
    assert isinstance(cookie["secure"], bool)
    assert "httpOnly" in cookie
    assert isinstance(cookie["httpOnly"], bool)
    if "expiry" in cookie:
        assert cookie.get("expiry") is None
    assert "sameSite" in cookie
    assert isinstance(cookie["sameSite"], str)

    assert cookie["name"] == "foo"
    assert cookie["value"] == "bar"


def test_get_named_cookie(session, url):
    session.url = url("/common/blank.html")
    clear_all_cookies(session)

    # same formatting as Date.toUTCString() in javascript
    utc_string_format = "%a, %d %b %Y %H:%M:%S"
    a_day_from_now = (datetime.now(timezone.utc) + timedelta(days=1)).strftime(utc_string_format)
    session.execute_script("document.cookie = 'foo=bar;expires=%s'" % a_day_from_now)

    result = get_named_cookie(session, "foo")
    cookie = assert_success(result)
    assert isinstance(cookie, dict)

    assert "name" in cookie
    assert isinstance(cookie["name"], str)
    assert "value" in cookie
    assert isinstance(cookie["value"], str)
    assert "expiry" in cookie
    assert isinstance(cookie["expiry"], int)
    assert "sameSite" in cookie
    assert isinstance(cookie["sameSite"], str)

    assert cookie["name"] == "foo"
    assert cookie["value"] == "bar"
    # convert from seconds since epoch
    assert datetime.utcfromtimestamp(
        cookie["expiry"]).strftime(utc_string_format) == a_day_from_now


def test_duplicated_cookie(session, url, server_config, inline):
    new_cookie = {
        "name": "hello",
        "value": "world",
        "domain": server_config["browser_host"],
        "path": "/",
        "http_only": False,
        "secure": False
    }

    session.url = url("/common/blank.html")
    clear_all_cookies(session)

    session.set_cookie(**new_cookie)
    session.url = inline("""
      <script>
        document.cookie = '{name}=newworld; domain={domain}; path=/';
      </script>""".format(
        name=new_cookie["name"],
        domain=server_config["browser_host"]))

    result = get_named_cookie(session, new_cookie["name"])
    cookie = assert_success(result)
    assert isinstance(cookie, dict)

    assert "name" in cookie
    assert isinstance(cookie["name"], str)
    assert "value" in cookie
    assert isinstance(cookie["value"], str)
    assert "sameSite" in cookie
    assert isinstance(cookie["sameSite"], str)

    assert cookie["name"] == new_cookie["name"]
    assert cookie["value"] == "newworld"


@pytest.mark.parametrize("same_site", ["None", "Lax", "Strict"])
def test_get_cookie_with_same_site_flag(session, url, same_site):
    session.url = url("/common/blank.html", protocol="https")
    clear_all_cookies(session)

    session.execute_script("document.cookie = 'foo=bar;Secure;SameSite=%s'" % same_site)

    result = get_named_cookie(session, "foo")
    cookie = assert_success(result)
    assert isinstance(cookie, dict)

    assert "name" in cookie
    assert isinstance(cookie["name"], str)
    assert "value" in cookie
    assert isinstance(cookie["value"], str)
    assert "sameSite" in cookie
    assert isinstance(cookie["sameSite"], str)

    assert cookie["name"] == "foo"
    assert cookie["value"] == "bar"
    assert cookie["sameSite"] == same_site
