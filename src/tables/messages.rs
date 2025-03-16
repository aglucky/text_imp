use imessage_database::{
    tables::{
        messages::{Message, models::BubbleComponent}, 
        table::{get_connection, Table, AttributedBody}
    }, 
    util::dirs::default_db_path
};
use polars::prelude::*;
use std::error::Error;
use crate::CustomError;
use chrono::TimeZone;
use serde_json;

pub fn messages_to_df() -> Result<DataFrame, Box<dyn Error>> {
    let db = get_connection(&default_db_path())
        .map_err(|e| Box::new(CustomError(e.to_string())))?;
    let mut statement = Message::get(&db)
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Collect all messages into a Vec first
    let mut messages = Vec::new();
    let mut rows = statement
        .query_map([], |row| Ok(Message::from_row(row)))
        .map_err(|e| Box::new(CustomError(e.to_string())))?;

    // Process all messages
    while let Some(message) = rows.next() {
        let mut msg = Message::extract(message)
            .map_err(|e| Box::new(CustomError(e.to_string())))?;
        let _ = msg.generate_text(&db);
        messages.push(msg);
    }

    // Create essential columns
    let text = Series::new("text", messages.iter().map(|m| m.text.clone()).collect::<Vec<Option<String>>>());
    let is_from_me = Series::new("is_from_me", messages.iter().map(|m| m.is_from_me).collect::<Vec<bool>>());
    let handle_id = Series::new("handle_id", messages.iter().map(|m| m.handle_id).map(|id| id.unwrap_or(0)).collect::<Vec<i32>>());
    let chat_id = Series::new("chat_id", messages.iter().map(|m| m.chat_id).map(|id| id.unwrap_or(0)).collect::<Vec<i32>>());
    let guid = Series::new("guid", messages.iter().map(|m| m.guid.clone()).collect::<Vec<String>>());
    let thread_originator_guid = Series::new("thread_originator_guid", messages.iter().map(|m| m.thread_originator_guid.clone()).collect::<Vec<Option<String>>>());
    let thread_originator_part = Series::new("thread_originator_part", messages.iter().map(|m| m.thread_originator_part.clone()).collect::<Vec<Option<String>>>());
    let is_deleted = Series::new("is_deleted", messages.iter().map(|m| m.is_deleted()).collect::<Vec<bool>>());
    let group_title = Series::new("group_title", messages.iter().map(|m| m.group_title.clone()).collect::<Vec<Option<String>>>());

    // Add derived columns
    let service = Series::new("service_type", messages.iter().map(|m| format!("{:?}", m.service())).collect::<Vec<String>>());
    let variant = Series::new("variant", messages.iter().map(|m| format!("{:?}", m.variant())).collect::<Vec<String>>());
    let announcement = Series::new(
        "announcement_type", 
        messages.iter()
            .map(|m| m.get_announcement()
                .map(|a| format!("{:?}", a)))
            .collect::<Vec<Option<String>>>()
    );
    let expressive = Series::new(
        "expressive", 
        messages.iter()
            .map(|m| m.expressive_send_style_id.clone())
            .collect::<Vec<Option<String>>>()
    );
    let num_attachments = Series::new("num_attachments", messages.iter().map(|m| m.num_attachments).collect::<Vec<i32>>());
    let is_edited = Series::new("is_edited", messages.iter().map(|m| m.is_edited()).collect::<Vec<bool>>());
    let is_tapback = Series::new("is_tapback", messages.iter().map(|m| m.is_tapback()).collect::<Vec<bool>>());
    let is_reply = Series::new("is_reply", messages.iter().map(|m| m.is_reply()).collect::<Vec<bool>>());
    let has_replies = Series::new("has_replies", messages.iter().map(|m| m.has_replies()).collect::<Vec<bool>>());
    let is_url = Series::new("is_url", messages.iter().map(|m| m.is_url()).collect::<Vec<bool>>());
    let num_replies = Series::new("num_replies", messages.iter().map(|m| m.num_replies).collect::<Vec<i32>>());
    let body = Series::new(
        "body", 
        messages.iter()
            .map(|m| {
                let components = m.body()
                    .into_iter()
                    .filter_map(|component| match component {
                        BubbleComponent::Text(text) => Some(serde_json::json!({ 
                            "type": "text", 
                            "content": format!("{:?}", text)
                        })),
                        BubbleComponent::Attachment(meta) => Some(serde_json::json!({
                            "type": "attachment",
                            "guid": meta.guid.unwrap_or(""),
                            "name": meta.name.unwrap_or(""),
                            "dimensions": if meta.height.is_some() && meta.width.is_some() {
                                Some(serde_json::json!({
                                    "height": meta.height.unwrap(),
                                    "width": meta.width.unwrap()
                                }))
                            } else {
                                None
                            }
                        })),
                        BubbleComponent::App => Some(serde_json::json!({ "type": "app" })),
                        BubbleComponent::Retracted => Some(serde_json::json!({ "type": "retracted" }))
                    })
                    .collect::<Vec<_>>();
                serde_json::to_string(&components).unwrap_or_default()
            })
            .collect::<Vec<String>>()
    );
    
    let date = Series::new(
        "date", 
        messages.iter().map(|m| {
            let offset = &978307200;
            m.date(offset)
                .unwrap_or_else(|_| chrono::Local.timestamp_millis_opt(0).unwrap())
                .with_timezone(&chrono::Utc)
                .timestamp_nanos_opt()
                .unwrap_or(0)
        })
        .collect::<Vec<i64>>()
    ).cast(&DataType::Datetime(TimeUnit::Nanoseconds, Some("UTC".into())))
        .unwrap();

    let date_delivered = Series::new(
        "date_delivered",
        messages.iter().map(|m| {
            let offset = &978307200;
            m.date_delivered(offset)
                .unwrap_or_else(|_| chrono::Local.timestamp_millis_opt(0).unwrap())
                .with_timezone(&chrono::Utc)
                .timestamp_nanos_opt()
                .unwrap_or(0)
        })
        .collect::<Vec<i64>>()
    ).cast(&DataType::Datetime(TimeUnit::Nanoseconds, Some("UTC".into())))
        .unwrap();

    let date_read = Series::new(
        "date_read",
        messages.iter().map(|m| {
            let offset = &978307200;
            m.date_read(offset)
                .unwrap_or_else(|_| chrono::Local.timestamp_millis_opt(0).unwrap())
                .with_timezone(&chrono::Utc)
                .timestamp_nanos_opt()
                .unwrap_or(0)
        })
        .collect::<Vec<i64>>()
    ).cast(&DataType::Datetime(TimeUnit::Nanoseconds, Some("UTC".into())))
        .unwrap();

    let cols: Vec<Series> = vec![
        date, text, is_from_me, handle_id, chat_id, guid, thread_originator_guid, thread_originator_part,
        service, variant, expressive, announcement, num_attachments, is_deleted, group_title,
        is_edited, is_tapback, is_reply, num_replies, date_delivered, date_read, is_url, has_replies, body
    ];

    DataFrame::new(cols)
        .map_err(|e| Box::new(CustomError(e.to_string())) as Box<dyn Error>)
}

