mod saver;

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*, 
    utils::command::BotCommands,
    // types::{InlineKeyboardButton, InlineKeyboardMarkup},
};
use crate::saver::get_video;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start, 
    ReceiveFullName, 
    ReceiveUrlYoutube{
        full_name: String,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display all command and help")]
    Help,
    #[command(description = "start progress")]
    Start,
    #[command(description = "cancel download")]
    Cancel,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    
    Dispatcher::builder(bot, message_handler())
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

fn message_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start)),
             )
        .branch(case![Command::Cancel].endpoint(cancel));


    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveFullName].endpoint(receive_fullname))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query().branch(
        case![State::ReceiveUrlYoutube { full_name }].endpoint(receive_product_selection),
    );

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn start(bot: Bot,dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! type your url of youtube video").await?;
    // get_video(msg, bot).await;
    dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! what's your full name").await?;
    Ok(())
}

async fn cancel(bot: Bot,dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, format!("cancelled, good by {}", msg.chat.username().unwrap_or(""))).await?;
    dialogue.exit().await?;
    Ok(())
}


async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "invalied").await?;
    Ok(())
}

async fn receive_fullname(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(full_name) => {
            bot.send_message(msg.chat.id, "Your space will be fly and send in minutes").await;
            get_video(msg, bot, full_name).await;
            // let products = ["Apple", "Banana", "Orange", "Potato"]
            //     .map(|product| InlineKeyboardButton::callback(product, product));

            // bot.send_message(msg.chat.id, "Select a product:")
            //     .reply_markup(InlineKeyboardMarkup::new([products]))
            //     .await?;
            // dialogue.update(State::ReceiveUrlYoutube { full_name }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
        }
    }
    Ok(())
}


async fn receive_product_selection(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String, // Available from `State::ReceiveProductChoice`.
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(product) = &q.data {
        bot.send_message(
            dialogue.chat_id(),
            format!("{full_name}, product '{product}' has been purchased successfully!"),
        )
        .await?;
        dialogue.exit().await?;
    }

    Ok(())
}
