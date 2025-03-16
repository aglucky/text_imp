use imessage_database::{
    tables::{
        attachment::Attachment,
        table::{get_connection, Table}
    },
    util::dirs::default_db_path
};
use polars::prelude::*;
use std::error::Error;
use crate::CustomError;

pub fn attachments_to_df() -> Result<DataFrame, Box<dyn Error>> {
    let db = get_connection(&default_db_path())
        .map_err(|e| Box::new(CustomError(e.to_string())))?;
    let mut statement = Attachment::get(&db)
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Collect all attachments into a Vec first
    let mut attachments = Vec::new();
    let mut rows = statement
        .query_map([], |row| Ok(Attachment::from_row(row)))
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Process all attachments
    while let Some(attachment) = rows.next() {
        let attachment = Attachment::extract(attachment)
            .map_err(|e| Box::new(CustomError(e.to_string())))?;
        attachments.push(attachment);
    }

    // Create columns based on Attachment struct fields
    let rowid = Series::new("rowid", attachments.iter().map(|a| a.rowid as i32).collect::<Vec<i32>>());
    let filename = Series::new("filename", attachments.iter().map(|a| a.filename.clone()).collect::<Vec<Option<String>>>());
    let uti = Series::new("uti", attachments.iter().map(|a| a.uti.clone()).collect::<Vec<Option<String>>>());
    let mime_type = Series::new("mime_type", attachments.iter().map(|a| a.mime_type.clone()).collect::<Vec<Option<String>>>());
    let transfer_name = Series::new("transfer_name", attachments.iter().map(|a| a.transfer_name.clone()).collect::<Vec<Option<String>>>());
    let is_sticker = Series::new("is_sticker", attachments.iter().map(|a| (a.is_sticker as i64) != 0).collect::<Vec<bool>>());
    let emoji_description = Series::new("emoji_description", attachments.iter().map(|a| a.emoji_description.clone()).collect::<Vec<Option<String>>>());

    // Add derived columns using Attachment methods
    let path = Series::new("path", attachments.iter().map(|a| {
        a.path().map(|p| p.to_string_lossy().into_owned())
    }).collect::<Vec<Option<String>>>());

    let extension = Series::new("extension", attachments.iter().map(|a| {
        a.extension().map(|e| e.to_string())
    }).collect::<Vec<Option<String>>>());

    let display_filename = Series::new("display_filename", attachments.iter().map(|a| {
        Some(a.filename().to_string())
    }).collect::<Vec<Option<String>>>());

    let file_size = Series::new("file_size", attachments.iter().map(|a| {
        Some(a.file_size())
    }).collect::<Vec<Option<String>>>());

    // Create the DataFrame from all columns
    let cols: Vec<Series> = vec![
        rowid, filename, uti, mime_type, transfer_name, emoji_description,
        is_sticker, path, extension, display_filename, file_size
    ];

    DataFrame::new(cols)
        .map_err(|e| Box::new(CustomError(e.to_string())) as Box<dyn Error>)
} 