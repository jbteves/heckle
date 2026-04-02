import pyheckle


def test_known_seed_output():
    assert pyheckle.to_spongebob_case("hello world") == "HELlo wORLd"


def test_empty_string():
    assert pyheckle.to_spongebob_case("") == ""


def test_non_alpha_passthrough():
    assert pyheckle.to_spongebob_case("123 !@#") == "123 !@#"


def test_caseless_alpha_passthrough():
    # CJK characters have no case mapping; they pass through unchanged
    # and must not affect the run counter.
    result = pyheckle.to_spongebob_case("漢字hello")
    assert result.startswith("漢字")
    ascii_alpha = [c for c in result if c.isascii() and c.isalpha()]
    assert len(ascii_alpha) == 5  # "hello"


def test_non_alpha_does_not_break_run():
    # Non-alpha chars interspersed with alpha chars must not reset the run counter.
    result = pyheckle.to_spongebob_case("a1b2c3d4e")
    alpha = [c for c in result if c.isalpha() and (c.isupper() or c.islower())]
    run = 1
    for a, b in zip(alpha, alpha[1:]):
        if a.isupper() == b.isupper():
            run += 1
            assert run <= 3, f"run exceeded 3 in {alpha}"
        else:
            run = 1


def test_ascii_output_length_matches_input():
    # ASCII case conversions are always 1:1, so lengths must match.
    s = "Hello, World! 123"
    assert len(pyheckle.to_spongebob_case(s)) == len(s)


def test_content_preserved():
    # The alpha content lowercased must match the input alpha content lowercased.
    s = "hello world"
    result = pyheckle.to_spongebob_case(s)
    lowered = "".join(c for c in result if c.isascii() and c.isalpha()).lower()
    assert lowered == "helloworld"
