use hashbrown::HashMap;

use grok;
use pyo3::prelude::*;

#[pyclass(name = "Grok", module = "rustgrok")]
pub struct GrokPy {
    pub obj: grok::Grok,
    pub pattern: Option<grok::Pattern>,
}

impl Default for GrokPy {
    fn default() -> Self {
        Self {
            obj: grok::Grok::with_default_patterns(),
            pattern: None,
        }
    }
}

#[pymethods]
impl GrokPy {
    #[new]
    pub fn __new__() -> Self {
        return GrokPy::default();
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
        if self.pattern.is_none() {
            return res;
        }
        let matches: Option<grok::Matches> = self.pattern.as_ref().unwrap().match_against(target);
        if matches.is_none() {
            return res;
        }
        for (k, v) in matches.unwrap().iter() {
            res.insert(k.to_string(), v.to_string());
        }
        return res;
    }

    pub fn add_pattern(&mut self, name: &str, pattern: &str) {
        self.obj.add_pattern(name, pattern);
    }
}

impl From<GrokPy> for grok::Grok {
    fn from(_: GrokPy) -> Self {
        grok::Grok::default()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustgrok(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GrokPy>()?;
    Ok(())
}
