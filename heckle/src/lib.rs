//! heckle is a meme case conversion library.
//!
//! This library exists to facilitate lulz with meme cases, such as spONgEBoB caSE and BILLY MAYS
//! MODE. It is intended to be unicode-aware, internally consistent-ish, and reasonably useless.
//!
//! # Definition of a word boundary
//! Unlike the popular and useful heck library, we do not care about word boundaries. In fact, in
//! BILLY MAYS MODE, we drop non-alphabetic, non-whitespace characters because we don't
//! think it matters anyway.
//!
//! # Cases contained in this library:
//! ## sPoNGeBoB CAsE
//! Randomly alternates letter casing with no more than 3 consecutive same-case characters. If you
//! want more than 3 characters, too bad, I hardcoded it.
//! Also it's not really random because I hardedcoded the RNG to always be seeded with 42.
//! That's the answer to life, the universe, and everything, and I think it's the answer to meme
//! case RNG seeding, too.
//!
//! ```
//! use heckle::ToSpongebobCase;
//! assert_eq!("hello world".to_spongebob_case(), "HELlo wORLd");
//! ```
//!
//! ## BILLY MAYS MODE
//! Uppercases every character that has a case, dropping non-alphabetic, non-whitespace characters. BUT WAIT! you
//! might say; well, too bad, no more.
//!
//! ```
//! use heckle::ToBillyMaysMode;
//! assert_eq!("wait, there's more!!!".to_billy_mays_mode(), "WAIT THERES MORE");
//! ```
//!
//! # A Note on AI Usage
//! This is pure vibe-coded slop. Thanks, Claude!

mod billy_mays;
mod spongebob;

pub use billy_mays::{BillyMays, ToBillyMaysMode};
pub use spongebob::{Spongebob, ToSpongebobCase};

/// Returns `true` if `ch` has a case conversion that actually changes the codepoint.
///
/// Some characters report `is_uppercase()` / `is_lowercase()` as `true` yet have no
/// actual mapping in Rust's tables (e.g. mathematical Fraktur/script letters). This
/// helper gates on whether the conversion *actually produces a different codepoint*.
/// The conversion may be multi-char (ligatures that decompose on uppercasing, etc.).
/// Used by Billy Mays, which uppercases every char that can be uppercased.
#[inline]
pub(crate) fn char_has_case(ch: char) -> bool {
    ch.to_uppercase().ne(std::iter::once(ch)) || ch.to_lowercase().ne(std::iter::once(ch))
}

/// Returns `true` if `ch` has a *single-codepoint* case conversion that changes it.
///
/// Stricter than `char_has_case`: ligatures and other chars whose `to_uppercase` /
/// `to_lowercase` decompose into multiple codepoints return `false`. Used by Spongebob
/// so that the run counter and case conversion are always 1-char-in / 1-char-out,
/// keeping the "no more than 3 consecutive same-case chars" invariant unambiguous.
#[inline]
pub(crate) fn char_has_simple_case(ch: char) -> bool {
    let mut upper = ch.to_uppercase();
    let mut lower = ch.to_lowercase();
    let u = upper.next().unwrap_or(ch);
    let l = lower.next().unwrap_or(ch);
    // Both conversions must be single codepoints, and at least one must differ from ch.
    upper.next().is_none() && lower.next().is_none() && (u != ch || l != ch)
}
