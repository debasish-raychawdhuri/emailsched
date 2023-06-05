use crate::extraction::Event;
use colored::*;

pub trait Displayable {
    fn display(&self);
}

fn clock_face(event: &Event) -> char {
    let hour = event.date_time.hour;
    let minute = event.date_time.minute;
    let rounded_hour = if minute > 45 { hour + 1 } else { hour };
    let rounded_hour = if rounded_hour > 12 {
        rounded_hour - 12
    } else {
        rounded_hour
    };
    let rounded_minute = if minute > 45 || minute < 15 { 0 } else { 30 };
    if rounded_minute == 0 {
        return char::from_u32('\u{1F54F}' as u32 + rounded_hour as u32).unwrap();
    } else {
        return char::from_u32('\u{1F55B}' as u32 + rounded_hour as u32).unwrap();
    }
}

impl Displayable for Event {
    fn display(&self) {
        println!(
            "{}-{}-{} {}:{}",
            format!("\u{1F4C5}{}", self.date_time.year).yellow(),
            format!("{:02}", self.date_time.month).yellow(),
            format!("{:02}", self.date_time.day).yellow(),
            format!("{}{:02}", clock_face(self), self.date_time.hour).green(),
            format!("{:02}", self.date_time.minute).green(),
        );
        let sanitized = self.sanitize();
        for key in self.keys.iter() {
            let value = sanitized.text.get(key).unwrap();
            println!("           {}:{}", key.cyan(), value.white());
        }
    }
}
