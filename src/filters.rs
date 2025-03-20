use chrono::NaiveDateTime;
use askama::Result as AskamaResult; 

pub fn format_date(value: &NaiveDateTime) -> AskamaResult<String> {
    Ok(value.format("%d-%m-%Y %I:%M %p").to_string()) // `%I` for 12-hour format, `%p` for AM/PM
}