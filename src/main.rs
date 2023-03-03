use std::collections::VecDeque;

use irc::{client::prelude::*, error::Error};
use futures::prelude::*;
mod module;


#[tokio::main]
async fn main() -> Result<(), Error>{
    let max_len = 100;

    let config = Config::load("config.toml")?;
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    
    let mut message_buf: VecDeque<Message> = VecDeque::with_capacity(max_len);
    while let Some(message) = stream.next().await.transpose()? {
        print!("{}",message);
        let response = module::handle(&message, &message_buf);

        if let Some((target,msg))= response {
            print!("{}",message);
            sender.send_privmsg(target,msg)?;
        } 

        if message_buf.len() < max_len {
            message_buf.push_front(message);
        }
        else {
            let _ = message_buf.pop_back();
            message_buf.push_front(message);
        }
    }

    Ok(())
}



