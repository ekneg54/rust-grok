use std::collections::HashMap;

use grok;
use pyo3::prelude::*;

#[pyclass]
pub struct Grok {
    pub obj: grok::Grok,
    pub pattern: Option<grok::Pattern>,
}

impl Default for Grok {
    fn default() -> Self {
        Self {
            obj: grok::Grok::with_default_patterns(),
            pattern: None,
        }
    }
}

#[pymethods]
impl Grok {
    #[new]
    pub fn __new__() -> Self {
        return Grok::default();
    }

    pub fn compile(&mut self, pattern_str: &str) {
        self.pattern = Some(
            self.obj
                .compile(pattern_str, false)
                .expect("Error while compiling"),
        );
    }

    pub fn match_against(&self, target: &str) -> HashMap<String, String> {
        let mut res: HashMap<String, String> = HashMap::new();
        if self.pattern.is_some() {
            let matches: Option<grok::Matches> =
                self.pattern.as_ref().unwrap().match_against(target);
            if matches.is_some() {
                for (k, v) in matches.unwrap().iter() {
                    res.insert(k.to_string(), v.to_string());
                }
                return res;
            } else {
                return res;
            }
        } else {
            return res;
        }
    }

    pub fn add_pattern(&mut self, name: &str, pattern: &str) {
        self.obj.add_pattern(name, pattern);
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustgrok(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Grok>()?;
    Ok(())
}
