mod aks_prime;

#[cfg(feature = "python")]
mod python {
    use crate::aks_prime;
    use pyo3::prelude::*;

    /// AKS primality test
    #[pyfunction]
    fn prime(n: u64) -> PyResult<bool> {
        Ok(aks_prime::aks(n))
    }

    /// A Python module implemented in Rust.
    #[pymodule]
    fn aks(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(prime, m)?)?;
        Ok(())
    }
}
