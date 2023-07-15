use teloxide::{prelude::*, utils::command::BotCommands};
use std::process::Command;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command[rename_rule = "lowercase", description = "These commands are supported:"]]
enum CommandBot {
    #[command(description = "display this text")]
    Help, 
    #[command(description = "handle username")]
    Username(String),
    #[command(description = "handle a username and a age.", parse_with = "split")]
    UsernameAndAge {username: String, age: u8},
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        CommandBot::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        CommandBot::Username(username) => {
            let lscommand = Command::new("ls").Spawn()
            log::info!("Starting command bot...");
            bot.send_message(msg.chat.id, format!("Your username is @{lscommand}")).await?
        }
        CommandBot::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("@{username} and @{age}.")).await?
        } 
    };
    Ok(())
}

