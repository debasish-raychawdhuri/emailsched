use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;

use crate::{
    config::{DateTimeField, EmailTemplate, TextField},
    email::Email,
};

pub fn match_all_templates(email: &Email, config: &crate::config::Config) -> Option<Event> {
    for template in config.email_templates.iter() {
        if match_email_template(email, template) {
            return extract_event(
                email,
                &template.date_time_field_formats,
                &template.text_field_formats.clone().unwrap_or_default(),
            );
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
) -> Option<NaiveDateTime> {
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
    let date_time_str = format!(
        "{:02}-{:02}-{:02} {:02}:{:02}:{:02}",
        year % 100,
        month,
        day,
        hour,
        minute,
        second
    );
    let date_time = NaiveDateTime::parse_from_str(&date_time_str, "%y-%m-%d %H:%M:%S").ok()?;
    Some(date_time)
}

pub struct ExtractedText {
    pub text: Vec<String>,
    pub name: String,
}

fn extract_text_field(
    Email { subject, body }: Email,
    TextField { name, regex }: TextField,
) -> Option<ExtractedText> {
    let re = Regex::new(&regex).unwrap();
    let text = re.find(&body)?.as_str();
    Some(ExtractedText {
        text: vec![text.to_string()],
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

#[derive(Debug, Clone)]
pub struct Event {
    date_time: NaiveDateTime,
    text: Vec<String>,
}

fn extract_all_text_fields(email: &Email, text_fields: &[TextField]) -> Vec<ExtractedText> {
    text_fields
        .iter()
        .filter_map(|tf| extract_text_field(email.clone(), tf.clone()))
        .collect()
}

fn extract_event(
    email: &Email,
    date_time_field_formats: &[DateTimeField],
    text_fields: &[TextField],
) -> Option<Event> {
    let date_time = date_time_field_formats
        .iter()
        .find_map(|dtf| extract_date_time(email.clone(), dtf.clone()));
    let text = extract_all_text_fields(email, text_fields)
        .iter()
        .map(|et| et.text.clone())
        .flatten()
        .collect();
    date_time.map(|dt| Event {
        date_time: dt,
        text,
    })
}
