use irc::proto::Message;
use std::collections::VecDeque;

pub const PATTERN: &str = "^\\$rtf([cm])$";
pub const NAME: &str = "rtfm";
pub const USAGE: &str = "Usage: $rtf[cm]\r\nThis tells you read the f-ing manual or chat, whichever is chosen";

pub fn rtfm(captures: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {
    let c_or_m = captures.get(1).unwrap().as_str();

    if c_or_m == "c" {
        "Read the f-ing chat".to_string()
    }
    else {
        "Read the f-ing manual".to_string()
    }
}
