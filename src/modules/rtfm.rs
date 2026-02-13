use irc::proto::Message;
use std::collections::VecDeque;

pub const PATTERN: &str = "^\\$rtf([cm])( (?P<nick>[^\\s]+))?$";
pub const NAME: &str = "rtfm";
pub const USAGE: &str = "Usage: $rtf[cm] <nick>\r\nThis tells you read the f-ing manual or chat, whichever is chosen";

pub fn rtfm(captures: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {
    let c_or_m = captures.get(1).unwrap().as_str();

    if let Some(nick) = captures.get(2) {
        if c_or_m == "c" {
            format!("{}: Read the f-ing chat", nick.as_str())
        }
        else {
            format!("{}: Read the f-ing manual", nick.as_str())
        }
    }
    else {
        if c_or_m == "c" {
            "Read the f-ing chat".to_string()
        }
        else {
            "Read the f-ing manual".to_string()
        }
    }
}
