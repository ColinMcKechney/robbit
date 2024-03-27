use std::collections::VecDeque;
use irc::proto::{Message, Command::*};
use regex::Regex;
use rand::prelude::Rng;

const LENNYS:[&str;12] = ["( ͡° ͜ʖ ͡°)","( ͠° ͟ʖ ͡°)","ᕦ( ͡° ͜ʖ ͡°)ᕤ","( ͡° ͜ʖ ͡°)","( ͡~ ͜ʖ ͡°)","( ͡o ͜ʖ ͡o)","͡° ͜ʖ ͡ -","( ͡͡ ° ͜ ʖ ͡ °)","( ͡ ͡° ͡°  ʖ ͡° ͡°)","(ง ͠° ͟ل͜ ͡°)ง","( ͡° ͜ʖ ͡ °)","( ͡°╭͜ʖ╮͡° )"];
const PATTERN: &str = "^\\$[Ll]enny\\s*(?P<text>.*)$";
const USAGE: &str = "Usage: ![Ll]enny
Displays a Lenny face ( ͡° ͜ʖ ͡°)";

pub fn mod_message(message: &Message, message_buf: &VecDeque<Message>) -> Option<(String,String)> {
    let regex: Regex = Regex::new(PATTERN).expect("Error creating regex");

    //checks if it was a PRIVMSG
    if let PRIVMSG(_,msg) = message.command.clone() {
        //checks if it was lenny
        if let Some(captures) = regex.captures(msg.as_str()) {

            let lenny = LENNYS[rand::thread_rng().gen_range(0..12)].to_string();
            let text = captures.get(1).unwrap().as_str();

            return Some((message.response_target().unwrap_or("#lug").to_string(),format!("{} {}", lenny, text)));
        }
    }

    None
}

pub fn usage(message: &Message) -> (String,String) {
    //prints a usage, not sure when I'm gonna use this but let's see
    (message.response_target().unwrap_or("#lug").to_string(),USAGE.to_string())
}


