use rand::prelude::Rng;

const PHRASES:[&str;24] = [
    "Go back to your cronjobs",
    "Don't you have a student machine to fork bomb?",
    "If your poor excuse for a bot is GPT4 then I'm Alan Turing",
    "You put semicolons in your python code",
    "You put your curly brackets on a separate line",
    "Just stop now, you're not getting the guru point",
    "OS is going to eat you alive",
    "L + Ratio",
    "You probably think it makes more sense to index at 1",
    "Matlab user",
    "The one extra point isn't gonna save your grade",
    "Jupyter Notebooks user",
    "You definitely leave zombie processes when you exit VSCode",
    "It's the honor code not the honor suggestion",
    "Wow another bot that responds when you say hello, how unique",
    "Zahm may be closed, but you still give off Zahm vibes",
    "They really let anyone in here huh?",
    "You're the tech bro who doesn't actually know how to code",
    "You wear a sweater when it's 70 outside",
    "Go take a shower I can smell you through the screen",
    "Don't try to hard you'll hurt yourself",
    "I know fish with better coding skills than you",
    "Nepo baby",
    "this student uses LIGHT MODE in vscode"
];

pub fn join_rude(nick: &str, channel: &str) -> Option<(String,String)> {

    let rude_message: &str = PHRASES[rand::thread_rng().gen_range(0..PHRASES.len())];
    let complete_message: String = nick.to_string() + ": " + rude_message;

    Some((channel.to_string(), complete_message))
}


