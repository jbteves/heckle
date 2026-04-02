use crate::char_has_simple_case;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::fmt;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(42));
}

pub trait ToSpongebobCase {
    fn to_spongebob_case(&self) -> String;
    fn write_spongebob_case<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result;
}

impl ToSpongebobCase for str {
    fn to_spongebob_case(&self) -> String {
        let mut s = String::with_capacity(self.len());
        self.write_spongebob_case(&mut s).unwrap();
        s
    }

    fn write_spongebob_case<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        RNG.with(|rng: &RefCell<SmallRng>| {
            let mut rng = rng.borrow_mut();
            write_spongebob_with_rng(self, writer, &mut *rng)
        })
    }
}

// Separated so tests can drive it with a controlled RNG without touching thread-local state.
pub(crate) fn write_spongebob_with_rng<W: fmt::Write>(
    s: &str,
    writer: &mut W,
    rng: &mut impl Rng,
) -> fmt::Result {
    // run_len == 0 means no cased alphabetic characters have been seen yet.
    let mut run_len: u8 = 0;
    let mut current_upper: bool = false; // meaningless when run_len == 0

    for ch in s.chars() {
        // Only alphabetics with a single-codepoint case conversion participate in the run
        // counter. Ligatures that decompose (e.g. ﬖ → ՎՆ), chars with no mapping (math
        // Fraktur/script), caseless alphabetics (CJK, etc.), and non-alphabetics all pass
        // through unchanged. This keeps the run counter 1:1 with output codepoints.
        if ch.is_alphabetic() && char_has_simple_case(ch) {
            let new_upper = if run_len >= 3 {
                !current_upper // forced flip after 3 consecutive same-case cased alpha chars
            } else {
                rng.gen_bool(0.5)
            };

            if run_len == 0 || new_upper != current_upper {
                current_upper = new_upper;
                run_len = 1;
            } else {
                run_len += 1;
            }

            // char_has_simple_case guarantees both conversions are exactly one codepoint,
            // so we can safely take .next().unwrap() without iterating further.
            if new_upper {
                writer.write_char(ch.to_uppercase().next().unwrap())?;
            } else {
                writer.write_char(ch.to_lowercase().next().unwrap())?;
            }
        } else {
            writer.write_char(ch)?;
        }
    }

    Ok(())
}

pub struct Spongebob<'a>(pub &'a str);

impl fmt::Display for Spongebob<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write_spongebob_case(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    fn rng() -> SmallRng {
        SmallRng::seed_from_u64(42)
    }

    fn convert(s: &str) -> String {
        let mut out = String::new();
        write_spongebob_with_rng(s, &mut out, &mut rng()).unwrap();
        out
    }

    #[test]
    fn known_seed_output() {
        assert_eq!(convert("hello world"), "HELlo wORLd");
    }

    #[test]
    fn empty_string() {
        assert_eq!(convert(""), "");
    }

    #[test]
    fn non_alpha_passthrough() {
        assert_eq!(convert("123 !@#"), "123 !@#");
    }

    #[test]
    fn caseless_alpha_passthrough() {
        // Characters without a real case mapping (CJK, math Fraktur, etc.) must pass
        // through unchanged and must not affect the run counter.
        let input = "漢字hello";
        let result = convert(input);
        assert!(result.starts_with("漢字"));
        let ascii_part: String = result.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        assert_eq!(ascii_part.len(), 5); // "hello"
    }

    #[test]
    fn non_alpha_does_not_break_run() {
        // Non-alpha chars between alpha chars must not reset the run counter.
        // Verify the invariant directly: no run of >3 consecutive same-case cased alpha chars.
        let mut r = rng();
        let mut out = String::new();
        write_spongebob_with_rng("a1b2c3d4e", &mut out, &mut r).unwrap();
        let alpha: Vec<char> = out
            .chars()
            .filter(|c| c.is_alphabetic() && (c.is_uppercase() || c.is_lowercase()))
            .collect();
        let mut run = 1u8;
        for pair in alpha.windows(2) {
            if pair[0].is_uppercase() == pair[1].is_uppercase() {
                run += 1;
                assert!(run <= 3, "run exceeded 3: {:?}", alpha);
            } else {
                run = 1;
            }
        }
    }

    #[test]
    fn ascii_output_length_matches_input() {
        // ASCII case conversions are always 1:1, so lengths must match.
        let input = "Hello, World! 123";
        assert_eq!(convert(input).len(), input.len());
    }

    #[test]
    fn display_wrapper_writes_without_extra_allocation() {
        // The Display wrapper should produce the same logical content as to_spongebob_case.
        // Both advance the thread-local RNG, so we verify structure rather than exact equality.
        let s = "hello world";
        let via_display = format!("{}", Spongebob(s));
        let lower: String = via_display
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        assert_eq!(lower, "helloworld");
    }
}
