use heckle::{ToBillyMaysMode, ToSpongebobCase};
use pyo3::prelude::*;

/// Convert a string to Spongebob Case.
///
/// Randomly alternates letter casing with no more than 3 consecutive
/// same-case characters. Seeded deterministically at 42 per thread.
#[pyfunction]
fn to_spongebob_case(s: &str) -> String {
    s.to_spongebob_case()
}

/// Convert a string to Billy Mays Mode.
///
/// Drops all non-alphabetic, non-whitespace characters, uppercases
/// everything, collapses whitespace runs, and trims each line.
#[pyfunction]
fn to_billy_mays_mode(s: &str) -> String {
    s.to_billy_mays_mode()
}

#[pymodule]
fn pyheckle(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_spongebob_case, m)?)?;
    m.add_function(wrap_pyfunction!(to_billy_mays_mode, m)?)?;
    Ok(())
}
