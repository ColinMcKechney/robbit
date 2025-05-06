use std::collections::VecDeque;
use irc::proto::{Message, Command::*};
use regex::Regex;

//list of available modules, add mod [MODULE_NAME]; when you complete a new module
//is this the best way to do this? probably not
mod modules;

use modules::{bully, lenny, join_rude, grass, noemo, ttb, help, repo,rtfm, kick};

type ModuleFunc = fn(regex::Captures, &Message, &VecDeque<Message>)->String;
const NUM_MODS:usize = 9;


const MODULES: [(&str, ModuleFunc);NUM_MODS] = [(lenny::PATTERN, lenny::mod_message), (bully::PATTERN, bully::mod_message), (grass::PATTERN, grass::touch_grass), (noemo::PATTERN, noemo::no_emo), (ttb::PATTERN, ttb::time_to_baby), (help::PATTERN, help::help), (repo::PATTERN, repo::link), (rtfm::PATTERN, rtfm::rtfm), (kick::PATTERN, kick::mod_message)];
const MODULE_USAGE: [(&str, &str); NUM_MODS] = [(lenny::NAME, lenny::USAGE), (bully::NAME, bully::USAGE), (grass::NAME, grass::USAGE), (noemo::NAME, noemo::USAGE), (ttb::NAME, ttb::USAGE), (help::NAME, help::USAGE), (repo::NAME, repo::USAGE),(rtfm::NAME, rtfm::USAGE), (kick::NAME, kick::USAGE)];

pub fn build_modules() -> Result<Vec<(Regex, ModuleFunc)>, regex::Error> {
    let mut regex_array: Vec<(Regex, ModuleFunc)> = Vec::with_capacity(NUM_MODS);
    for x in 0..MODULES.len() {
        let regex = regex::Regex::new(MODULES[x].0)?;
        regex_array.push((regex, MODULES[x].1));
    }
    Ok(regex_array)
}

pub fn handle(modules: &Vec<(Regex, ModuleFunc)>, message: &Message, message_buf: &VecDeque<Message>) -> Option<(String,String)> {

    match &message.command {
        PRIVMSG(_,msg) => for (regex, function) in modules{
                            if let Some(captures) = regex.captures(msg.as_str()) {
                                return Some((message.response_target().unwrap_or("#lug").to_string(), function(captures, message, message_buf)));
                            }
                        },
        //JOIN(ref channel,_,_) => return join_rude::join_rude(message.source_nickname().unwrap_or("unknown user"), channel.as_str()),
        _ => ()
    }

    None
}
