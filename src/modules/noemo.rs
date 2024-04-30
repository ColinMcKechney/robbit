use irc::proto::Message;
use std::collections::VecDeque;

pub const PATTERN: &str = "^\\$noemo( (?P<nick>[^\\s]+))?$";
pub const NAME: &str = "noemo";
pub const USAGE: &str = "Usage: $noemo <nick>\r\nThis tells the user identified by nick to not be emo, nick is optional";

pub fn no_emo(captures : regex::Captures, message: &Message, _: &VecDeque<Message>) -> String {
    let complete_message;
    if let Some(person) = captures.get(2) {
        complete_message = format!("{}: Don't be so emo. Take a deep breath and lighten up", person.as_str());
    }
    else {
        complete_message = format!("{} thinks you shouldn't be so emo. Take a deep breath and lighten up",
                                   message.source_nickname().unwrap_or("unknown_nick").to_string());
    }
    complete_message
}

