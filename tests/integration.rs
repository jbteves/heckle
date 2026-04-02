use heckle::{ToBillyMaysMode, ToSpongebobCase};
use proptest::prelude::*;

/// Mirror of the crate-private `char_has_simple_case`: single-codepoint conversions only.
fn char_has_simple_case(ch: char) -> bool {
    let mut upper = ch.to_uppercase();
    let mut lower = ch.to_lowercase();
    let u = upper.next().unwrap_or(ch);
    let l = lower.next().unwrap_or(ch);
    upper.next().is_none() && lower.next().is_none() && (u != ch || l != ch)
}

// ---------------------------------------------------------------------------
// Spongebob Case — property tests
// ---------------------------------------------------------------------------

proptest! {
    // Only cased alphabetic characters participate in the run counter.
    // Caseless alphabetics (CJK, etc.) and non-alphabetics are irrelevant to the run.
    #[test]
    fn spongebob_no_more_than_3_consecutive_same_case(s in "\\PC*") {
        let result = s.to_spongebob_case();
        let mut run_len = 0u8;
        let mut last_upper: Option<bool> = None;
        for ch in result.chars() {
            if ch.is_alphabetic() && char_has_simple_case(ch) {
                let is_upper = ch.is_uppercase();
                if last_upper == Some(is_upper) {
                    run_len += 1;
                    prop_assert!(
                        run_len <= 3,
                        "run of {} consecutive {:?} chars in {:?} (from {:?})",
                        run_len,
                        if is_upper { "upper" } else { "lower" },
                        result,
                        s
                    );
                } else {
                    run_len = 1;
                    last_upper = Some(is_upper);
                }
            }
        }
    }

    // Non-alphabetic characters must appear in the output in the same relative order.
    // (Alphabetic chars may expand to multiple codepoints on case conversion, but any
    // non-alphabetic codepoints produced by that expansion are filtered out, so they
    // cannot appear in the output either.)
    #[test]
    fn spongebob_preserves_non_alphabetic_chars(s in "\\PC*") {
        let result = s.to_spongebob_case();
        let input_non_alpha: Vec<char> = s.chars().filter(|c| !c.is_alphabetic()).collect();
        let output_non_alpha: Vec<char> = result.chars().filter(|c| !c.is_alphabetic()).collect();
        prop_assert_eq!(input_non_alpha, output_non_alpha);
    }

    // The set of truly-cased alphabetic characters (those with a real case mapping),
    // normalised to lowercase, must be identical in input and output.
    #[test]
    fn spongebob_preserves_cased_alpha_content(s in "[a-zA-Z ]*") {
        let result = s.to_spongebob_case();
        let input_lower: String = s.chars()
            .filter(|c| c.is_alphabetic() && char_has_simple_case(*c))
            .flat_map(|c| c.to_lowercase())
            .collect();
        let output_lower: String = result.chars()
            .filter(|c| c.is_alphabetic() && char_has_simple_case(*c))
            .flat_map(|c| c.to_lowercase())
            .collect();
        prop_assert_eq!(input_lower, output_lower);
    }

    // Alphabetics without a real case mapping must pass through Spongebob unchanged.
    #[test]
    fn spongebob_caseless_alpha_unchanged(s in "\\PC*") {
        let result = s.to_spongebob_case();
        let input_caseless: Vec<char> = s.chars()
            .filter(|c| c.is_alphabetic() && !char_has_simple_case(*c))
            .collect();
        let output_caseless: Vec<char> = result.chars()
            .filter(|c| c.is_alphabetic() && !char_has_simple_case(*c))
            .collect();
        prop_assert_eq!(input_caseless, output_caseless);
    }
}

// ---------------------------------------------------------------------------
// Billy Mays Mode — property tests
// ---------------------------------------------------------------------------

proptest! {
    // Every alphabetic char in the output must already be at its uppercase ceiling:
    // `ch.to_uppercase()` returns the char itself. This holds for:
    //   - Regular uppercase letters (A-Z, etc.)
    //   - Caseless alphabetics (CJK, etc.) — to_uppercase is identity
    //   - Characters that claim a case but have no actual mapping (math Fraktur, etc.)
    // Output must also contain no punctuation, digits, or other non-alphabetic/whitespace.
    #[test]
    fn billy_mays_output_only_contains_valid_chars(s in "\\PC*") {
        let result = s.to_billy_mays_mode();
        for ch in result.chars() {
            let valid = (ch.is_alphabetic() && ch.to_uppercase().eq(std::iter::once(ch)))
                || ch == ' '
                || ch == '\n';
            prop_assert!(
                valid,
                "unexpected char {:?} in {:?} (from {:?})",
                ch, result, s
            );
        }
    }

    #[test]
    fn billy_mays_is_idempotent(s in "\\PC*") {
        let once = s.to_billy_mays_mode();
        let twice = once.to_billy_mays_mode();
        prop_assert_eq!(&once, &twice, "not idempotent for input {:?}", s);
    }

    #[test]
    fn billy_mays_no_leading_or_trailing_spaces_per_line(s in "\\PC*") {
        let result = s.to_billy_mays_mode();
        for line in result.split('\n') {
            prop_assert!(
                !line.starts_with(' '),
                "leading space on line {:?} in {:?}",
                line, result
            );
            prop_assert!(
                !line.ends_with(' '),
                "trailing space on line {:?} in {:?}",
                line, result
            );
        }
    }

    #[test]
    fn billy_mays_no_consecutive_spaces(s in "\\PC*") {
        let result = s.to_billy_mays_mode();
        prop_assert!(
            !result.contains("  "),
            "consecutive spaces in {:?} (from {:?})",
            result, s
        );
    }

    #[test]
    fn billy_mays_preserves_newline_count(s in "[a-zA-Z \n]*") {
        let input_newlines = s.chars().filter(|&c| c == '\n').count();
        let result = s.to_billy_mays_mode();
        let output_newlines = result.chars().filter(|&c| c == '\n').count();
        prop_assert_eq!(input_newlines, output_newlines);
    }
}
