use std::collections::HashMap;

use grok;
use pyo3::prelude::*;

#[pyclass]
pub struct Grok {
    pub obj: grok::Grok,
    pub pattern: grok::Pattern,
}

#[pymethods]
impl Grok {
    #[new]
    pub fn __new__(pattern_str: &str) -> PyResult<Self> {
        let mut obj = grok::Grok::with_default_patterns();
        let pattern: grok::Pattern = obj
            .compile(pattern_str, false)
            .expect("Error while compiling");

        Ok(Self { obj, pattern })
    }

    pub fn match_against(&self, target: &str) -> HashMap<String, String> {
        let matches = self
            .pattern
            .match_against(target)
            .expect("No matches found!");
        let mut res: HashMap<String, String> = HashMap::new();
        for (k, v) in matches.iter() {
            res.insert(k.to_string(), v.to_string());
        }
        return res;
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustgrok(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Grok>()?;
    Ok(())
}
