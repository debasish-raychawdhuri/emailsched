use crate::extraction::Event;
use colored::*;

pub trait Displayable {
    fn display(&self);
}

impl Displayable for Event {
    fn display(&self) {
        println!(
            "{}-{}-{} {}:{}",
            format!("{}{}", '\u{23f0}', self.date_time.year).yellow(),
            format!("{:02}", self.date_time.month).yellow(),
            format!("{:02}", self.date_time.day).yellow(),
            format!("{:02}", self.date_time.hour).green(),
            format!("{:02}", self.date_time.minute).green(),
        );
        for (key, value) in self.sanitize().text.iter() {
            println!("           {}:{}", key.cyan(), value.white());
        }
    }
}
