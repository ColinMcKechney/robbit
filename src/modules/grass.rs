use irc::proto::Message;
use std::collections::VecDeque;

pub const PATTERN: &str = "\\$grass (?P<nick>[^\\s]+)";

pub fn touch_grass(captures: regex::Captures, message: &Message, _: &VecDeque<Message>) -> String {

    let grass_toucher = captures.get(1).unwrap().as_str();

    let complete_message = format!("{} thinks you should go outside and touch some grass, {}",
                                   message.source_nickname().unwrap_or("unknown_nick").to_string(),
                                   grass_toucher);

   complete_message
}
