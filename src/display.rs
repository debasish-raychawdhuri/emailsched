use crate::extraction::Event;
use colored::*;

pub trait Displayable {
    fn display(&self);
}

impl Displayable for Event {
    fn display(&self) {
        println!(
            "{}-{}-{} {}:{}",
            format!("{}", self.date_time.year).yellow(),
            format!("{}", self.date_time.month).yellow(),
            format!("{}", self.date_time.day).yellow(),
            format!("{}", self.date_time.hour).green(),
            format!("{}", self.date_time.minute).green(),
        );
        println!(
            "           {}",
            self.sanitize().text.get("Title").unwrap().white()
        );
    }
}
