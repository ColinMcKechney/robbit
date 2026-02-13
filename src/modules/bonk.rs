use std::collections::VecDeque;
use irc::proto::Message;


pub const USAGE: &str = "Usage: $bonk <nick>\r\nPut the person identified as nick in horny jail";

pub const NAME: &str = "bonk";

pub const PATTERN: &str = "^\\$bonk( (?P<nick>[^\\s]+))?$";

pub fn bonk(captures: regex::Captures, _message: &Message, _message_buf: &VecDeque<Message>) -> String {
    if let Some(nick) = captures.get(2) {
        format!("bonk! {} go to horny jail!", nick.as_str())
    }
    else {
        format!("bonk! go to horny jail!")
    }
}
