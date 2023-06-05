use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::extraction::Event;

pub trait EventManager {
    fn new_event(&mut self, event: Event);
}

pub struct JSEventManager {
    pub data_file: String,
    cached_events: HashMap<EventKey, Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct EventKey {
    year: i32,
    month: u32,
    day: u32,
    title: String,
}

impl JSEventManager {
    pub fn new(data_file: String) -> Self {
        Self {
            data_file,
            cached_events: HashMap::new(),
        }
    }

    pub fn load_events(&mut self) {
        let file = std::fs::File::open(&self.data_file).unwrap();
        let reader = std::io::BufReader::new(file);
        let events: Vec<Event> = serde_json::from_reader(reader).unwrap();
        let mut cached_events = HashMap::new();
        events.iter().for_each(|event| {
            cached_events.insert(
                EventKey {
                    year: event.date_time.year,
                    month: event.date_time.month,
                    day: event.date_time.day,
                    title: event.text.get("title").unwrap().to_string(),
                },
                event.clone(),
            );
        });
        self.cached_events = cached_events;
    }

    pub fn save_events(&self) {
        let file = std::fs::File::create(&self.data_file).unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self.cached_events).unwrap();
    }
}

impl EventManager for JSEventManager {
    fn new_event(&mut self, event: Event) {
        self.cached_events.insert(
            EventKey {
                year: event.date_time.year,
                month: event.date_time.month,
                day: event.date_time.day,
                title: event.text.get("title").unwrap().to_string(),
            },
            event,
        );
    }
}
