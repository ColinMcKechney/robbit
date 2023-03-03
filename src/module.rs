use std::collections::VecDeque;
use irc::proto::{Message, Command::*};

//list of available modules, add mod [MODULE_NAME]; when you complete a new module 
mod lenny;


type ModuleFunc = fn(&Message, &VecDeque<Message>)->Option<(String, String)>;
const NUM_MODS:usize = 1;


const MOD_FUNCS: [ModuleFunc;NUM_MODS] = [lenny::Lenny::mod_message];
pub fn handle(message: &Message, message_buf: &VecDeque<Message>) -> Option<(String,String)> {

    if let PRIVMSG(_,_) = message.command.clone() {
        for function in MOD_FUNCS{
            let response = function(message, message_buf);
            if response.is_some() {
                return response;
            }
        }
    }
    

    None
}

pub trait Module {
    fn usage(message: &Message) -> (String,String);
    fn mod_message(message: &Message, message_buf: &VecDeque<Message>) -> Option<(String,String)>;
}