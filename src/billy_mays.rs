use crate::char_has_case;
use std::fmt;

pub trait ToBillyMaysMode {
    fn to_billy_mays_mode(&self) -> String;
    fn write_billy_mays_mode<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result;
}

impl ToBillyMaysMode for str {
    fn to_billy_mays_mode(&self) -> String {
        let mut s = String::with_capacity(self.len());
        self.write_billy_mays_mode(&mut s).unwrap();
        s
    }

    fn write_billy_mays_mode<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        write_billy_mays(self, writer)
    }
}

pub(crate) fn write_billy_mays<W: fmt::Write>(s: &str, writer: &mut W) -> fmt::Result {
    let mut first_line = true;
    for line in s.split('\n') {
        if !first_line {
            writer.write_char('\n')?;
        }
        first_line = false;
        write_billy_mays_line(line, writer)?;
    }
    Ok(())
}

fn write_billy_mays_line<W: fmt::Write>(line: &str, writer: &mut W) -> fmt::Result {
    // pending_space defers whitespace emission so trailing spaces are never written.
    // wrote_any prevents leading spaces from being emitted.
    let mut pending_space = false;
    let mut wrote_any = false;

    for ch in line.chars() {
        if ch.is_alphabetic() {
            if pending_space {
                writer.write_char(' ')?;
                pending_space = false;
            }

            if char_has_case(ch) {
                // Has a real case mapping: uppercase it. Filter non-alphabetic codepoints
                // from the expansion (e.g. combining marks) so the output is idempotent —
                // a combining mark dropped here won't reappear on a second pass.
                for c in ch.to_uppercase() {
                    if c.is_alphabetic() {
                        writer.write_char(c)?;
                    }
                }
            } else {
                // No real case mapping (CJK, math Fraktur/script, etc.): emit as-is.
                writer.write_char(ch)?;
            }

            wrote_any = true;
        } else if ch.is_whitespace() {
            // ch is never '\n' here since we split on '\n'.
            // Only arm pending_space once something has been written (trims leading whitespace).
            if wrote_any {
                pending_space = true;
            }
        }
        // Non-alphabetic, non-whitespace: drop.
    }
    // If pending_space is still true here, trailing whitespace is intentionally not flushed.

    Ok(())
}

pub struct BillyMays<'a>(pub &'a str);

impl fmt::Display for BillyMays<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write_billy_mays_mode(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!("".to_billy_mays_mode(), "");
    }

    #[test]
    fn basic_conversion() {
        assert_eq!(
            "wait, there's more!!! buy now".to_billy_mays_mode(),
            "WAIT THERES MORE BUY NOW"
        );
    }

    #[test]
    fn drops_punctuation() {
        assert_eq!("hello!!!".to_billy_mays_mode(), "HELLO");
    }

    #[test]
    fn collapses_whitespace() {
        assert_eq!("hello   world".to_billy_mays_mode(), "HELLO WORLD");
    }

    #[test]
    fn trims_leading_whitespace() {
        assert_eq!("   hello".to_billy_mays_mode(), "HELLO");
    }

    #[test]
    fn trims_trailing_whitespace() {
        assert_eq!("hello   ".to_billy_mays_mode(), "HELLO");
    }

    #[test]
    fn preserves_newlines() {
        assert_eq!("hello\nworld".to_billy_mays_mode(), "HELLO\nWORLD");
    }

    #[test]
    fn preserves_empty_lines() {
        assert_eq!("hello\n\nworld".to_billy_mays_mode(), "HELLO\n\nWORLD");
    }

    #[test]
    fn trims_lines_around_newlines() {
        assert_eq!(
            "  hello!!  \n\n  buy now!!!  ".to_billy_mays_mode(),
            "HELLO\n\nBUY NOW"
        );
    }

    #[test]
    fn idempotent_ascii() {
        let cases = [
            "wait, there's more!!!",
            "  hello   world  ",
            "hello\n\n  world  ",
            "123 !@# abc",
            "",
        ];
        for input in cases {
            let once = input.to_billy_mays_mode();
            let twice = once.to_billy_mays_mode();
            assert_eq!(once, twice, "not idempotent for: {:?}", input);
        }
    }

    #[test]
    fn caseless_alpha_preserved() {
        // CJK characters are alphabetic but have no case; they pass through unchanged.
        let result = "hello 世界".to_billy_mays_mode();
        assert_eq!(result, "HELLO 世界");
    }

    #[test]
    fn display_wrapper() {
        assert_eq!(format!("{}", BillyMays("hello, world!")), "HELLO WORLD");
    }
}
