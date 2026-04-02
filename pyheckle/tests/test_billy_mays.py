import pyheckle


def test_empty_string():
    assert pyheckle.to_billy_mays_mode("") == ""


def test_basic_conversion():
    assert pyheckle.to_billy_mays_mode("wait, there's more!!! buy now") == "WAIT THERES MORE BUY NOW"


def test_drops_punctuation():
    assert pyheckle.to_billy_mays_mode("hello!!!") == "HELLO"


def test_collapses_whitespace():
    assert pyheckle.to_billy_mays_mode("hello   world") == "HELLO WORLD"


def test_trims_leading_whitespace():
    assert pyheckle.to_billy_mays_mode("   hello") == "HELLO"


def test_trims_trailing_whitespace():
    assert pyheckle.to_billy_mays_mode("hello   ") == "HELLO"


def test_preserves_newlines():
    assert pyheckle.to_billy_mays_mode("hello\nworld") == "HELLO\nWORLD"


def test_preserves_empty_lines():
    assert pyheckle.to_billy_mays_mode("hello\n\nworld") == "HELLO\n\nWORLD"


def test_trims_lines_around_newlines():
    assert pyheckle.to_billy_mays_mode("  hello!!  \n\n  buy now!!!  ") == "HELLO\n\nBUY NOW"


def test_idempotent_ascii():
    cases = [
        "wait, there's more!!!",
        "  hello   world  ",
        "hello\n\n  world  ",
        "123 !@# abc",
        "",
    ]
    for s in cases:
        once = pyheckle.to_billy_mays_mode(s)
        twice = pyheckle.to_billy_mays_mode(once)
        assert once == twice, f"not idempotent for: {s!r}"


def test_caseless_alpha_preserved():
    # CJK characters are alphabetic but have no case; they pass through unchanged.
    assert pyheckle.to_billy_mays_mode("hello 世界") == "HELLO 世界"


def test_display_wrapper():
    # In Python there's no Display trait; the function is the interface.
    assert pyheckle.to_billy_mays_mode("hello, world!") == "HELLO WORLD"
