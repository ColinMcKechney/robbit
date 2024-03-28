use irc::proto::Message;
use std::collections::VecDeque;

pub const PATTERN: &str = "\\$help(?: (.*))?";
pub const NAME: &str = "help";
pub const USAGE: &str = "You just used it";


pub fn help(captures: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {

    if let Some(command) = captures.get(1) {
        let command_str = command.as_str();

        for (name, usage_str) in super::super::MODULE_USAGE {
            if command_str == name {
                return usage_str.to_string();
            }
        }

        format!("Unknown command: {}", command_str)

    }
    else {
        let mut commands = String::new();
        for (command, _) in super::super::MODULE_USAGE {
            commands += format!("{}, ", command).as_str();
        }
        commands.pop();

        commands
    }
}
