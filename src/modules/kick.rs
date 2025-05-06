use irc::proto::Message;
use std::collections::VecDeque;
use regex::Regex;

pub const PATTERN: &str = "^\\$kick (?P<nick>[^\\s]+)";
pub const USAGE: &str = "";
pub const NAME: &str = "kick";



pub fn mod_message(captures: regex::Captures, message: &Message, _message_buf: &VecDeque<Message>) -> String {
    let to_be_kicked = captures.get(1).unwrap().as_str();

    format!("/kick {}", to_be_kicked)
}

pub fn usage(message: &Message) -> (String, String) {
    (message.response_target().unwrap_or("#lug").to_string(), USAGE.to_string())
}

pub fn bad_user(user: &str, channel: &str) -> Option<(String, String)> {
    let regex = match Regex::new(".*[dD]ick[rR]ider.*") {
        Ok(r) => r,
        Err(_) => {
            return None;
        }
    };

    if regex.is_match(user) {
        Some((channel.to_string(), format!("/kick {}", user)))
    }
    else {
        None
    }
}
