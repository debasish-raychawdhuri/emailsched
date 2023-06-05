use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DateTimeField {
    pub regex: String,
    pub year_group: usize,
    pub month_group: usize,
    pub day_group: usize,
    pub hour_group: usize,
    pub minute_group: usize,
    pub second_group: Option<usize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextField {
    pub name: String,
    pub regex: String,
}

#[derive(Deserialize, Debug)]
pub struct EmailTemplate {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub date_time_field_formats: Vec<DateTimeField>,
    pub text_field_formats: Option<Vec<TextField>>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub imap_server: String,
    pub imap_port: u16,
    pub data_store: String,
    pub email_templates: Vec<EmailTemplate>,
}

pub fn config_dir() -> String {
    let home = std::env::var("HOME").unwrap();
    format!("{}/.config/emailsched", home)
}

pub fn read_config() -> Config {
    let config_str = std::fs::read_to_string(format!("{}/Config.toml", config_dir())).unwrap();
    let config = toml::from_str(&config_str);
    if let Ok(config) = config {
        config
    } else {
        panic!("Error reading config file: {:?}", config);
    }
}
