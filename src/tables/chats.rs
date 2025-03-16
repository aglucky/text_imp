use imessage_database::{
    tables::{
        chat::Chat,
        table::{get_connection, Table}
    },
    util::dirs::default_db_path
};
use polars::prelude::*;
use std::error::Error;
use crate::CustomError;

pub fn chats_to_df() -> Result<DataFrame, Box<dyn Error>> {
    let db = get_connection(&default_db_path())
        .map_err(|e| Box::new(CustomError(e.to_string())))?;
    let mut statement = Chat::get(&db)
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Collect all chats into a Vec first
    let mut chats = Vec::new();
    let mut rows = statement
        .query_map([], |row| Ok(Chat::from_row(row)))
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Process all chats
    while let Some(chat) = rows.next() {
        let chat = Chat::extract(chat)
            .map_err(|e| Box::new(CustomError(e.to_string())))?;
        chats.push(chat);
    }

    // Create columns based on Chat struct fields
    let rowid = Series::new("rowid", chats.iter().map(|c| c.rowid as i32).collect::<Vec<i32>>());
    let chat_identifier = Series::new("chat_identifier", chats.iter().map(|c| c.chat_identifier.clone()).collect::<Vec<String>>());
    let service_name = Series::new("service_name", chats.iter().map(|c| c.service_name.clone()).collect::<Vec<Option<String>>>());
    let display_name = Series::new("display_name", chats.iter().map(|c| c.display_name.clone()).collect::<Vec<Option<String>>>());

    // Add derived columns using Chat methods
    let name = Series::new("name", chats.iter().map(|c| {
        Some(c.name().to_string())
    }).collect::<Vec<Option<String>>>());

    let resolved_display_name = Series::new("resolved_display_name", chats.iter().map(|c| {
        c.display_name().map(|n| n.to_string())
    }).collect::<Vec<Option<String>>>());

    // Create the DataFrame from all columns
    let cols: Vec<Series> = vec![
        rowid, chat_identifier, service_name, display_name,
        name, resolved_display_name
    ];

    DataFrame::new(cols)
        .map_err(|e| Box::new(CustomError(e.to_string())) as Box<dyn Error>)
} 