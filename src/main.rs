mod config;
mod display;
mod email;
mod events;
mod extraction;
mod ncurses;
use clap::{command, Command};
use display::Displayable;
use email::fetch_inbox_top;
use extraction::Event;

use crate::events::EventManager;
fn main() {
    let matches = command!()
        .subcommand(Command::new("refresh").about("Refresh from the inbox"))
        .subcommand(Command::new("list").about("List events"))
        .subcommand_required(true)
        .get_matches();
    match matches.subcommand() {
        Some(("refresh", _)) => refresh(),
        Some(("list", _)) => list(),
        _ => {}
    }
}
fn list() {
    let config = config::read_config();
    let data_store = format!("{}/{}", config::config_dir(), config.data_store);
    let mut event_store = events::JSEventManager::new(data_store);
    event_store.load_events();
    let mut events: Vec<&Event> = event_store.cached_events.values().collect();
    events.sort_by(|a, b| {
        if a.date_time.year > b.date_time.year {
            std::cmp::Ordering::Greater
        } else if a.date_time.year < b.date_time.year {
            std::cmp::Ordering::Less
        } else if a.date_time.month > b.date_time.month {
            std::cmp::Ordering::Greater
        } else if a.date_time.month < b.date_time.month {
            std::cmp::Ordering::Less
        } else if a.date_time.day > b.date_time.day {
            std::cmp::Ordering::Greater
        } else if a.date_time.day < b.date_time.day {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });
    for event in events {
        event.display();
    }
}
fn refresh() {
    let config = config::read_config();
    let data_store = format!("{}/{}", config::config_dir(), config.data_store);
    let mut event_store = events::JSEventManager::new(data_store);
    event_store.load_events();
    for message in fetch_inbox_top(&config).unwrap().iter() {
        if let Some(message) = message {
            let event = extraction::match_all_templates(message, &config);
            if let Some(event) = event {
                event_store.new_event(event);
            }
        }
    }
    event_store.save_events();
}
