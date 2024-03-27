use std::collections::VecDeque;
use irc::proto::{Message, Command::PRIVMSG};
use regex::Regex;
use rand::prelude::Rng;


const USAGE: &str = "Usage: !bully <nick>
This bullies the user identified by nick.
";

const BULLY_PHRASES:[&str;12] = [
    "'s day has been ruined by your message, ",
    " wants to return to monke, but not if you're coming, too, ",
    " knows how much of a duck-banging degenerate you are, ",
    " hopes you order pizza from Modern Market, but then you realize you have an interview to go to that you're about to be late to, so you frantically rush to it before realizing it's over Zoom, so you pull out your laptop and search through your email, but can't find the link, before finally discovering it 2 whole minutes later, making you late to your interview, which you fail by the way, after which you remember you ordered pizza which, even though cold, would still be enough to lift your spirits up a little, except you find it was taken by someone else, Modern Market has closed, and you are left with nothing but dread, disgust, and misery, ",
    " believes you're too incompetent to know that you're being bullied, ",
    " doesn't care about your race, sex, or age... or anything about you really, ",
    " has more maidens than you, ",
    "'s faith in society has plummeted since meeting you, ",
    " think you're about as good as the dining hall food, ",
    " gives eMoTiOnAL dAmAgE to ",
    " has a confession to make. You're ugly, ",
    " thinks you were probably the pilot of Ever Given when it clogged the Suez Canal, "
];

pub const PATTERN: &str = "^\\$bully (?P<nick>[^\\s]+)";

pub fn mod_message(captures: regex::Captures, message: &Message, _message_buf: &VecDeque<Message>) -> Option<(String,String)> {

    let bully_message: String = BULLY_PHRASES[rand::thread_rng().gen_range(0..BULLY_PHRASES.len())].to_string();
    let to_be_bullied = captures.get(1).unwrap().as_str();

    let complete_message = message.source_nickname().unwrap_or("unknown_nick").to_string() + bully_message.as_str() + to_be_bullied;

    Some((message.response_target().unwrap_or("#lug").to_string(), complete_message))

}

pub fn usage(message: &Message) -> (String, String) {
    (message.response_target().unwrap_or("#lug").to_string(), USAGE.to_string())
}
