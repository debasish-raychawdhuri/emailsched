use std::collections::{BTreeMap, HashMap};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    config::{DateTimeField, EmailTemplate, TextField},
    email::Email,
};

pub fn match_all_templates(email: &Email, config: &crate::config::Config) -> Option<Event> {
    for template in config.email_templates.iter() {
        if match_email_template(email, template) {
            if let Some(extraced_event) = extract_event(
                email,
                &template.date_time_field_formats,
                &template.text_field_formats.clone().unwrap_or_default(),
            ) {
                return Some(extraced_event);
            }
        }
    }

    None
}

pub fn match_email_template(email: &Email, template: &EmailTemplate) -> bool {
    let mut matches = false;
    if let Some(subject) = &template.subject {
        let sub_regex = Regex::new(subject).unwrap();
        matches = matches || sub_regex.is_match(&email.subject);
    }
    if let Some(body) = &template.body {
        let body_regex = Regex::new(body).unwrap();
        matches = matches || body_regex.is_match(&email.body);
    }
    matches
}

fn extract_date_time(
    Email {
        body,
        subject: _subject,
    }: Email,
    date_time_format: DateTimeField,
) -> Option<DateTime> {
    let re = Regex::new(&date_time_format.regex).unwrap();
    let captures = re.captures(&body)?;
    let year = captures.get(date_time_format.year_group)?.as_str();
    let month = captures.get(date_time_format.month_group)?.as_str();
    let day = captures.get(date_time_format.day_group)?.as_str();
    let hour = captures.get(date_time_format.hour_group)?.as_str();
    let minute = captures.get(date_time_format.minute_group)?.as_str();
    let second = captures
        .get(date_time_format.second_group.unwrap_or_default())?
        .as_str();

    let year = year.parse::<i32>().ok()?;
    let month = string_to_month_number(month)?;
    let day = day.parse::<u32>().ok()?;
    let hour = hour.parse::<u32>().ok()?;
    let minute = minute.parse::<u32>().ok()?;
    let second = second.parse::<u32>().ok().unwrap_or_default();

    let date_time = DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
    };
    Some(date_time)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedText {
    pub text: String,
    pub name: String,
}

fn extract_text_field(
    Email { subject: _, body }: Email,
    TextField { name, regex }: TextField,
) -> Option<ExtractedText> {
    let re = Regex::new(&regex).unwrap();
    let caps = re.captures(&body)?;
    let text = caps.get(1)?.as_str();
    Some(ExtractedText {
        text: text.to_string(),
        name,
    })
}

fn string_to_month_number(month: &str) -> Option<u32> {
    match month.parse::<u32>() {
        Ok(month) => return Some(month),
        Err(_) => match month[0..3].to_string().as_str() {
            "Jan" => Some(1),
            "Feb" => Some(2),
            "Mar" => Some(3),
            "Apr" => Some(4),
            "May" => Some(5),
            "Jun" => Some(6),
            "Jul" => Some(7),
            "Aug" => Some(8),
            "Sep" => Some(9),
            "Oct" => Some(10),
            "Nov" => Some(11),
            "Dec" => Some(12),
            _ => None,
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    pub day: u32,
    pub month: u32,
    pub year: i32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub date_time: DateTime,
    pub text: BTreeMap<String, String>,
    pub keys: Vec<String>,
}
impl Event {
    pub fn sanitize(&self) -> Event {
        let mut text: BTreeMap<String, String> = BTreeMap::new();

        let regex = Regex::new(r"\s+").unwrap();
        for (key, value) in self.text.iter() {
            text.insert(key.to_string(), {
                let line_text = value.replace("\n", " ").replace("\r", " ");
                regex.replace_all(&line_text, " ").to_string()
            });
        }
        Event {
            date_time: self.date_time.clone(),
            text,
            keys: self.keys.clone(),
        }
    }
}
fn extract_all_text_fields(
    email: &Email,
    text_fields: &[TextField],
) -> (BTreeMap<String, String>, Vec<String>) {
    let mut map = BTreeMap::new();
    let mut keys = vec![];
    text_fields.iter().for_each(|tf| {
        let text_field = extract_text_field(email.clone(), tf.clone());
        if let Some(text_field) = text_field {
            map.insert(text_field.name.clone(), text_field.text);
            keys.push(text_field.name);
        }
    });
    return (map, keys);
}

fn extract_event(
    email: &Email,
    date_time_field_formats: &[DateTimeField],
    text_fields: &[TextField],
) -> Option<Event> {
    let date_time = date_time_field_formats
        .iter()
        .find_map(|dtf| extract_date_time(email.clone(), dtf.clone()));
    let (text, keys) = extract_all_text_fields(email, text_fields);
    date_time.map(|dt| Event {
        date_time: dt,
        text,
        keys,
    })
}
