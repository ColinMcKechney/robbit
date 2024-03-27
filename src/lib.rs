use std::collections::VecDeque;
use irc::proto::{Message, Command::*};

//list of available modules, add mod [MODULE_NAME]; when you complete a new module
//is this the best way to do this? probably not
mod modules;

use modules::{bully, lenny, join_rude};

type ModuleFunc = fn(&Message, &VecDeque<Message>)->Option<(String, String)>;
const NUM_MODS:usize = 2;


const MOD_FUNCS: [ModuleFunc;NUM_MODS] = [lenny::mod_message, bully::mod_message];
pub fn handle(message: &Message, message_buf: &VecDeque<Message>) -> Option<(String,String)> {

    match message.command {
        PRIVMSG(_,_) => for function in MOD_FUNCS{
                            let response = function(message, message_buf);
                            if response.is_some() {
                                return response;
                            }
                        },
        JOIN(ref channel,_,_) => return join_rude::join_rude(message.source_nickname().unwrap_or("unknown user"), channel.as_str()),
        _ => ()
    }


    None
}
