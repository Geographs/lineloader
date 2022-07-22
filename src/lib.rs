use std::fs::File;
use std::io::{BufRead, BufReader};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read(file_name: &str) -> PyResult<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    )
}

/// A Python module implemented in Rust.
#[pymodule]
fn lineloader(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
