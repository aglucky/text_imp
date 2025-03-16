use imessage_database::{
    tables::{
        chat_handle::ChatToHandle,
        table::{get_connection, Cacheable}
    },
    util::dirs::default_db_path
};
use polars::prelude::*;
use std::error::Error;
use crate::CustomError;

pub fn chat_handles_to_df() -> Result<DataFrame, Box<dyn Error>> {
    let db = get_connection(&default_db_path())
        .map_err(|e| Box::new(CustomError(e.to_string())))?;
    
    // Get all chat-handle relationships using the cache
    let chat_handles = ChatToHandle::cache(&db)
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Convert the HashMap into vectors for the DataFrame
    let mut chat_ids = Vec::new();
    let mut handle_ids = Vec::new();

    for (chat_id, handles) in chat_handles {
        for handle_id in handles {
            chat_ids.push(chat_id as i32);
            handle_ids.push(handle_id as i32);
        }
    }

    // Create columns based on ChatToHandle data
    let chat_id = Series::new("chat_id", chat_ids);
    let handle_id = Series::new("handle_id", handle_ids);

    // Create the DataFrame from all columns
    let cols: Vec<Series> = vec![
        chat_id, handle_id
    ];

    DataFrame::new(cols)
        .map_err(|e| Box::new(CustomError(e.to_string())) as Box<dyn Error>)
} 