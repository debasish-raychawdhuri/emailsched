mod config;
mod email;
mod events;
mod extraction;
mod ncurses;
use email::fetch_inbox_top;
fn main() {
    // let config = config::read_config();
    // for template in config.email_templates.iter() {
    //     println!("{:?}", template);
    // }
    //
    // for message in fetch_inbox_top(&config).unwrap().iter() {
    //     if let Some(message) = message {
    //         let event = extraction::match_all_templates(message, &config);
    //         println!("{:?}", event);
    //     }
    // }
    ncurses::display();
}
