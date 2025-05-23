use irc::proto::Message;
use std::collections::VecDeque;
use chrono::{prelude::*, TimeDelta};

pub const PATTERN: &str = "^\\$ttb\\s*$";
pub const NAME: &str = "ttb";
pub const USAGE: &str = "Usage $ttb\r\nThis prints the number of days until pnutz's baby is due";


pub fn time_to_baby(_: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {
    let local_time: DateTime<Local> = Local::now();

    let birth_time: DateTime<Local> = Local.with_ymd_and_hms(2024, 10, 8, 00, 00, 00).unwrap();

    let difference = local_time - birth_time;

    let completed_message;
    /*if difference > TimeDelta::zero() {
        completed_message = format!("{} {} until pnutz's baby is due!", difference.num_days(), if difference.num_days() > 1 { "days"} else {"day"} );
    }
    else {
        completed_message = "They're past due!".to_string();
    }*/
    completed_message = format!("He's {} days old!", difference.num_days());


    completed_message
}
