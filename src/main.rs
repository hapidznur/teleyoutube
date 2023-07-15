use teloxide::{
    prelude::*, 
    payloads::SendMessaageSetters,
};
use std::error::Error;

fn message_handler(
    m: Message, 
    bot: Bot
) -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    let chat = &m.chat;
    let username = chat.username().map(String::from);

    if let Some(maybe_url) = m.text() {
        if maybe_url == "/start" {
            bot.send_message(chat.id, format!("Your username is send url instead")).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    teloxide::enable_logging!();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();
    
    Dispatcher::builder(bot, message_handler)
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
