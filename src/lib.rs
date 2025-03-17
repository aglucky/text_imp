use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use crate::tables::{
    messages::messages_to_df,
    attachments::attachments_to_df,
    chats::chats_to_df,
    chat_handles::chat_handles_to_df,
    handles::handles_to_df
};

mod tables;

#[derive(Debug)]
pub struct CustomError(pub String);

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[pymodule]
fn _core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_messages, m)?)?;
    m.add_function(wrap_pyfunction!(get_attachments, m)?)?;
    m.add_function(wrap_pyfunction!(get_chats, m)?)?;
    m.add_function(wrap_pyfunction!(get_chat_handles, m)?)?;
    m.add_function(wrap_pyfunction!(get_handles, m)?)?;
    
    // Add the MESSAGE_DB_PATH constant
    m.add("MESSAGE_DB_PATH", imessage_database::util::dirs::default_db_path().to_string_lossy().to_string())?;
    
    Ok(())
}

#[pyfunction]
fn get_messages() -> PyResult<PyDataFrame> {
    let df = messages_to_df()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn get_attachments() -> PyResult<PyDataFrame> {
    let df = attachments_to_df()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn get_chats() -> PyResult<PyDataFrame> {
    let df = chats_to_df()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn get_chat_handles() -> PyResult<PyDataFrame> {
    let df = chat_handles_to_df()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyDataFrame(df))
}

#[pyfunction]
fn get_handles() -> PyResult<PyDataFrame> {
    let df = handles_to_df()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyDataFrame(df))
} 