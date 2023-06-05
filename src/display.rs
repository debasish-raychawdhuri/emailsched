use crate::extraction::Event;
use colored::*;

pub trait Displayable {
    fn display(&self);
}

impl Displayable for Event {
    fn display(&self) {
        println!(
            "{}-{}-{} {}:{}",
            format!("\u{1F4C5}{}", self.date_time.year).yellow(),
            format!("{:02}", self.date_time.month).yellow(),
            format!("{:02}", self.date_time.day).yellow(),
            format!("\u{1f551}{:02}", self.date_time.hour).green(),
            format!("{:02}", self.date_time.minute).green(),
        );
        let sanitized = self.sanitize();
        for key in self.keys.iter() {
            let value = sanitized.text.get(key).unwrap();
            println!("           {}:{}", key.cyan(), value.white());
        }
    }
}
