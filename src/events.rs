use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::extraction::Event;

pub trait EventManager {
    fn new_event(&mut self, event: Event);
}

pub struct JSEventManager {
    pub data_file: String,
    pub cached_events: HashMap<String, Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct EventKey {
    year: i32,
    month: u32,
    day: u32,
    title: String,
}

impl EventKey {
    fn to_string(&self) -> String {
        format!("{}-{}-{}-{}", self.year, self.month, self.day, self.title)
    }
}

impl JSEventManager {
    pub fn new(data_file: String) -> Self {
        Self {
            data_file,
            cached_events: HashMap::new(),
        }
    }

    pub fn load_events(&mut self) {
        if let Ok(file) = std::fs::File::open(&self.data_file) {
            let reader = std::io::BufReader::new(file);
            let events: Vec<Event> = serde_json::from_reader(reader).unwrap();
            let mut cached_events = HashMap::new();
            events.iter().for_each(|event| {
                cached_events.insert(
                    EventKey {
                        year: event.date_time.year,
                        month: event.date_time.month,
                        day: event.date_time.day,
                        title: event.text.get("Title").unwrap().to_string(),
                    }
                    .to_string(),
                    event.clone(),
                );
            });
            self.cached_events = cached_events;
        }
    }

    pub fn save_events(&self) {
        let file = std::fs::File::create(&self.data_file).unwrap();
        let writer = std::io::BufWriter::new(file);
        let cached_events: Vec<Event> = self.cached_events.values().cloned().collect();
        serde_json::to_writer(writer, &cached_events).unwrap();
    }
}

impl EventManager for JSEventManager {
    fn new_event(&mut self, event: Event) {
        self.cached_events.insert(
            EventKey {
                year: event.date_time.year,
                month: event.date_time.month,
                day: event.date_time.day,
                title: event
                    .text
                    .get("Title")
                    .unwrap_or(&"".to_string())
                    .to_string(),
            }
            .to_string(),
            event,
        );
    }
}
