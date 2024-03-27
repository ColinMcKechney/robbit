use std::collections::VecDeque;
use irc::proto::Message;
use rand::prelude::Rng;

const LENNYS:[&str;12] = ["( ͡° ͜ʖ ͡°)","( ͠° ͟ʖ ͡°)","ᕦ( ͡° ͜ʖ ͡°)ᕤ","( ͡° ͜ʖ ͡°)","( ͡~ ͜ʖ ͡°)","( ͡o ͜ʖ ͡o)","͡° ͜ʖ ͡ -","( ͡͡ ° ͜ ʖ ͡ °)","( ͡ ͡° ͡°  ʖ ͡° ͡°)","(ง ͠° ͟ل͜ ͡°)ง","( ͡° ͜ʖ ͡ °)","( ͡°╭͜ʖ╮͡° )"];
pub const PATTERN: &str = "^\\$[Ll]enny\\s*(?P<text>.*)$";
const USAGE: &str = "Usage: ![Ll]enny
Displays a Lenny face ( ͡° ͜ʖ ͡°)";

pub fn mod_message(captures: regex::Captures, message: &Message, _message_buf: &VecDeque<Message>) -> Option<(String,String)> {
    let lenny = LENNYS[rand::thread_rng().gen_range(0..LENNYS.len())].to_string();
    let text = captures.get(1).unwrap().as_str();

    Some((message.response_target().unwrap_or("#lug").to_string(),format!("{} {}", lenny, text)))
}

pub fn usage(message: &Message) -> (String,String) {
    //prints a usage, not sure when I'm gonna use this but let's see
    (message.response_target().unwrap_or("#lug").to_string(),USAGE.to_string())
}


