use irc::proto::Message;
use std::collections::VecDeque;
use irc::proto::Command::*;

pub const PATTERN: &str = "^\\$history (?P<nick>[^\\s]+) (?P<amount>[0-9]+)";
pub const NAME: &str = "history";
pub const USAGE: &str = "Usage: $history <nick> <amount\r\nThis replays the last <amount> of messages from a user";

pub fn mod_message(captures: regex::Captures, _message: &Message, message_buf: &VecDeque<Message>) -> String {
    let amount: usize = captures.get(2).unwrap().as_str().parse().unwrap();
    let mut messages: Vec<String> = Vec::with_capacity(amount);
    let mut total_message: String = format!("No messages found for user: {}", captures.get(1).unwrap().as_str());

    for message in message_buf {
        if let Some(nick) = message.source_nickname() {
            if nick == captures.get(1).unwrap().as_str() && messages.len() < amount {
                match &message.command {
                    PRIVMSG(_,msg) => {
                        messages.push(msg.clone());
                    },
                    _ => ()
                };
            }
            if messages.len() == amount {
                break;
            }
        }
    }

    if messages.len() > 0 {
        messages.reverse();
        total_message = messages.join("\r\n");
    }
    total_message
}

pub fn usage(message: &Message) -> (String, String) {
    (message.response_target().unwrap_or("#lug").to_string(), USAGE.to_string())
}
