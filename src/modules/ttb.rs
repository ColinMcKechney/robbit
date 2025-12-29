use irc::proto::Message;
use std::collections::VecDeque;
use std::time::Duration;
use chrono::prelude::*;
use humantime;

pub const PATTERN: &str = "^\\$ttb\\s*$";
pub const NAME: &str = "ttb";
pub const USAGE: &str = "Usage $ttb\r\nThis prints the number of days until pnutz's baby is due";


pub fn time_to_baby(_: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {
    let local_time: DateTime<Local> = Local::now();

    let birth_time: DateTime<Local> = Local.with_ymd_and_hms(2024, 10, 8, 00, 00, 00).unwrap();

    let difference = local_time - birth_time;

    let completed_message;
    
    let human_difference = humantime::format_duration(Duration::from_secs(difference.num_seconds() as u64));
    completed_message = format!("He's {} old!", human_difference.to_string());


    completed_message
}
