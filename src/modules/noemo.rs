use irc::proto::Message;
use std::collections::VecDeque;

pub const PATTERN: &str = "\\$noemo (?P<nick>[^\\s]+)";

pub fn no_emo(captures: regex::Captures, message: &Message, _: &VecDeque<Message>) -> String {
    let emo_person = captures.get(1).unwrap().as_str();

    let complete_message = format!("{} thinks you shouldn't be so emo, {}. Take a deep breath and lighten up",
                                   message.source_nickname().unwrap_or("unknown_nick").to_string(),
                                   emo_person);
    complete_message
}

