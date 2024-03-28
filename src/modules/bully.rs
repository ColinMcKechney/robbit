use std::collections::VecDeque;
use irc::proto::Message;
use rand::prelude::Rng;


pub const USAGE: &str = "Usage: $bully <nick>\r\nThis bullies the user identified by nick.
";
pub const NAME: &str = "bully";

const BULLY_PHRASES:[&str;11] = [
    "'s day has been ruined by your message, ",
    " wants to return to monke, but not if you're coming, too, ",
    " knows how much of a duck-banging degenerate you are, ",
    " believes you're too incompetent to know that you're being bullied, ",
    " doesn't care about your race, sex, or age... or anything about you really, ",
    " has more maidens than you, ",
    "'s faith in society has plummeted since meeting you, ",
    " thinks you're about as good as the dining hall food, ",
    " gives eMoTiOnAL dAmAgE to ",
    " has a confession to make. You're ugly, ",
    " thinks you were probably the pilot of Ever Given when it clogged the Suez Canal, "
];

pub const PATTERN: &str = "^\\$bully (?P<nick>[^\\s]+)";

pub fn mod_message(captures: regex::Captures, message: &Message, _message_buf: &VecDeque<Message>) -> String {

    let bully_message: String = BULLY_PHRASES[rand::thread_rng().gen_range(0..BULLY_PHRASES.len())].to_string();
    let to_be_bullied = captures.get(1).unwrap().as_str();

    let complete_message = message.source_nickname().unwrap_or("unknown_nick").to_string() + bully_message.as_str() + to_be_bullied;

    complete_message

}

pub fn usage(message: &Message) -> (String, String) {
    (message.response_target().unwrap_or("#lug").to_string(), USAGE.to_string())
}
