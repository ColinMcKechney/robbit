use irc::proto::Message;
use std::collections::VecDeque;


pub const PATTERN: &str = "^\\$repo\\s*$";
pub const NAME: &str = "repo";
pub const USAGE: &str = "Usage: $repo\r\nThis gives the link to the robbit repo";
pub const REPO_LINK: &str = "https://github.com/ColinMcKechney/robbit.git";

pub fn link(_: regex::Captures, _: &Message, _: &VecDeque<Message>) -> String {
    format!("{}\r\nPRs are always welcome!", REPO_LINK)
}
