use irc::proto::Message;
use std::collections::VecDeque;
use std::time::Duration;
use chrono::prelude::*;
use humantime::{self, parse_rfc3339_weak};

pub const PATTERN: &str = "^\\$ttd\\s*(.*)$";
pub const NAME: &str = "ttd";
pub const USAGE: &str = "Usage $ttd <datetime>\r\nThis prints the amount of time until the specified datetime";

pub fn time_to_date(captures: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {
    let time_string = captures.get(1).unwrap().as_str();
    //Dates + Times (numeric)
    let requested_time = if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%D %R") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%D %I:%M %P") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%D %I:%M %p") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%m/%d/%Y %R") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%m/%d/%Y %I:%M %P") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%m/%d/%Y %I:%M %p") {
        date_time
    }
    //Dates (numeric)
    else if let Ok(date) = NaiveDate::parse_from_str(time_string, "%D") {
        NaiveDateTime::from(date)
    }
    else if let Ok(date) = NaiveDate::parse_from_str(time_string, "%m/%d/%Y") {
        NaiveDateTime::from(date)
    }
    else if let Ok(date) = NaiveDate::parse_from_str(time_string, "%F") {
        NaiveDateTime::from(date)
    }
    //Time (24hour)
    else if let Ok(time) = NaiveTime::parse_from_str(time_string, "%R") {
        Local::now().naive_local().date().and_time(time)
    }
    //Time (12hour)
    else if let Ok(time) = NaiveTime::parse_from_str(time_string, "%I:%M %P") {
        Local::now().naive_local().date().and_time(time)
    }
    else if let Ok(time) = NaiveTime::parse_from_str(time_string, "%I:%M %p") {
        Local::now().naive_local().date().and_time(time)
    }
    //Date + Time (written)
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%v %R") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%v %I:%M %P") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%v %I:%M %p") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%B %d, %y %R") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%B %d, %y %I:%M %p") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%B %d, %y %I:%M %P") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%B %d, %Y %R") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%B %d, %Y %I:%M %p") {
        date_time
    }
    else if let Ok(date_time) = NaiveDateTime::parse_from_str(time_string, "%B %d, %Y %I:%M %P") {
        date_time
    }
    //Date (written)
    else if let Ok(date) = NaiveDate::parse_from_str(time_string, "%v") {
        NaiveDateTime::from(date)
    }
    else if let Ok(date) = NaiveDate::parse_from_str(time_string, "%B %d, %y") {
        NaiveDateTime::from(date)
    }
    else if let Ok(date) = NaiveDate::parse_from_str(time_string, "%B %d, %Y") {
        NaiveDateTime::from(date)
    }
    //RFC3339 datetime
    else if let Ok(date_time) = parse_rfc3339_weak(time_string) {
        let tmp: DateTime<Local> = date_time.into();
        tmp.naive_local()
    }
    else {
        return "Invalid date given".to_string();
    };

    let current_time = Local::now().naive_local();

    let difference = requested_time - current_time;
    let human_difference = humantime::format_duration(Duration::from_secs(difference.num_seconds().abs() as u64));
    if difference.num_seconds() < 0 {
        format!("Was {} ago",human_difference.to_string())
    }
    else {
        format!("Is in {}", human_difference.to_string())
    }
}
