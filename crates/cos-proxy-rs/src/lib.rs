use pyo3::prelude::*;
use pyo3::types::{PyModule};
use tokio::runtime::Runtime;
use dotenv::dotenv;
use cos_proxy_core::run_server;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, Sender};

#[pyclass]
pub struct ServerHandle {
    stop_signal: Arc<Mutex<Option<Sender<()>>>>,
}

#[pymethods]
impl ServerHandle {
    pub fn stop(&self) -> PyResult<()> {
        if let Some(sender) = self.stop_signal.lock().unwrap().take() {
            sender.send(()).map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to stop server"))?;
        }
        Ok(())
    }
}


#[pyfunction]
pub fn jahallo(py: Python) -> PyResult<String>{
    Ok("jahallo".to_string())
}

#[pyfunction]
pub fn start_server() -> PyResult<String> {
    dotenv().ok();

    // Create a Tokio runtime to run the async function
    let rt = Runtime::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to create runtime: {}", e)))?;
    rt.block_on(async {
        run_server().await; // Assuming `run_server` is async
    });

    Ok("Server started".to_string())
}

// #[pyfunction]
// pub fn start_server(py: Python) -> PyResult<String>{
//     dotenv().ok();

//     run_server(); 

//     Ok("Server started".to_string())
// }

#[pymodule]
fn cos_proxy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(jahallo, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}