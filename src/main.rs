use std::collections::VecDeque;

use irc::{client::prelude::*, error::Error};
use futures::prelude::*;
use robbit::{build_modules, handle};

#[tokio::main]
async fn main() -> Result<(), Error>{
    let max_len = 100;

    let config = Config::load("config.toml")?;
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();


    let mut message_buf: VecDeque<Message> = VecDeque::with_capacity(max_len);
    let module_pair = build_modules().expect("Error building modules");
    while let Some(message) = stream.next().await.transpose()? {
        print!("{}",message);
        let response = handle(&module_pair, &message, &message_buf);

        if let Some((target,msg))= response {
            if msg.contains("/kick") {
                let message: Vec<&str> = msg.split(" ").collect();
                sender.send_kick(target, message[1], "")?;
            }
            else {
                print!("{}",message);
                sender.send_privmsg(target,msg)?;
            }
        }

        /*if message_buf.len() < max_len {
            message_buf.push_front(message);
        }
        else {
            let _ = message_buf.pop_back();
            message_buf.push_front(message);
        }*/
    }

    Ok(())
}



