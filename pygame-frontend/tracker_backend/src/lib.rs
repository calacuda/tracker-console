use ipc::TrackerIPC;
use pygame_coms::{Button, DisplayCursor, State};
use pyo3::prelude::*;

mod ipc;
mod pygame_coms;

/// A Python module implemented in Rust.
#[pymodule]
fn tracker_backend(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<TrackerIPC>()?;
    m.add_class::<Button>()?;
    m.add_class::<DisplayCursor>()?;
    m.add_class::<State>()?;
    // m.add_class()?;
    // m.add_class()?;
    // m.add_class()?;
    // m.add_class()?;
    // m.add_class()?;
    // m.add_class()?;
    // m.add_class()?;

    Ok(())
}
