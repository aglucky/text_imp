use imessage_database::{
    tables::{
        handle::Handle,
        table::{get_connection, Table}
    },
    util::dirs::default_db_path
};
use polars::prelude::*;
use std::error::Error;
use crate::CustomError;

pub fn handles_to_df() -> Result<DataFrame, Box<dyn Error>> {
    let db = get_connection(&default_db_path())
        .map_err(|e| Box::new(CustomError(e.to_string())))?;
    let mut statement = Handle::get(&db)
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Collect all handles into a Vec first
    let mut handles = Vec::new();
    let mut rows = statement
        .query_map([], |row| Ok(Handle::from_row(row)))
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Process all handles
    while let Some(handle) = rows.next() {
        let handle = Handle::extract(handle)
            .map_err(|e| Box::new(CustomError(e.to_string())))?;
        handles.push(handle);
    }

    // Create columns based on Handle struct fields
    let rowid = Series::new("rowid", handles.iter().map(|h| h.rowid as i32).collect::<Vec<i32>>());
    let id = Series::new("id", handles.iter().map(|h| h.id.clone()).collect::<Vec<String>>());
    let person_centric_id = Series::new("person_centric_id", handles.iter().map(|h| h.person_centric_id.clone()).collect::<Vec<Option<String>>>());

    // Create the DataFrame from all columns
    let cols: Vec<Series> = vec![
        rowid, id, person_centric_id
    ];

    DataFrame::new(cols)
        .map_err(|e| Box::new(CustomError(e.to_string())) as Box<dyn Error>)
} 