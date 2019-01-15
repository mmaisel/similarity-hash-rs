use pyo3::prelude::*;
// use pyo3::PyRawObject;
use fuzzy::ssdeep::{Signature, SimilarityHash};
use fuzzy::ssdeep::Hash as _Hash;


#[pyfunction]
fn hash(string: &str) -> PyResult<String> {
    let hash = _Hash::hash(string)?;
    Ok(hash.to_string())
}


#[pyfunction]
fn hash_from_file(path: &str) -> PyResult<String> {
    let hash = _Hash::hash_from_file(path)?;
    Ok(hash.to_string())
}


#[pyfunction]
fn compare(digest_a: &str, digest_b: &str) -> PyResult<u32> {
    let hash_a = Signature::from_string(digest_a);
    let hash_b = Signature::from_string(digest_b);
    let score = _Hash::compare(&hash_a, &hash_b);
    Ok(score)
}

// #[pyclass]
// pub struct Hash {
//     hash: _Hash,
// }

// #[pymethods]
// impl Hash {
// 
//     #[new]
//     fn __new__(obj: &PyRawObject) -> PyResult<()> {
//         obj.init(||
//             Hash {
//                 hash: _Hash::new(),
//             }
//         )
//     }
// 
//     fn update(&mut self, sample: &str) {
//         self.hash.hash_buffer(sample.as_bytes());
//     }
// 
//     fn update_byte(&mut self, byte: u8) {
//         self.hash.update(byte);
//     }
// 
//     fn digest(&mut self) -> PyResult<String> {
//         Ok(self.hash.digest().to_string())
//     }
// 
// }


#[pymodule]
fn ssdeep(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<Hash>()?;
    m.add_wrapped(wrap_function!(hash))?;
    m.add_wrapped(wrap_function!(hash_from_file))?;
    m.add_wrapped(wrap_function!(compare))?;
    Ok(())
}