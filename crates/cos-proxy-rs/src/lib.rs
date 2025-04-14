use pyo3::prelude::*;
use pyo3::types::{PyModule};
use tokio::runtime::Runtime;
use dotenv::dotenv;
use cos_proxy_core::run_server;


#[pyfunction]
pub fn jahallo(py: Python) -> PyResult<String>{
    Ok("jahallo".to_string())
}

#[pyfunction]
pub fn start_server(py: Python) -> PyResult<String>{
    dotenv().ok();

    run_server(); 

    Ok("Server started".to_string())
}

#[pymodule]
fn cos_proxy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(jahallo, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}