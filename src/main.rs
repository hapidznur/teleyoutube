use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*, 
    utils::command::BotCommands,
};

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
    #[command(description = "display this text")]
    Help,
    #[command(description = "display this text")]
    Start,
    #[command(description = "display this text")]
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

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! what's your full name").await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! what's your full name").await?;
    Ok(())
}

async fn cancel(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! what's your full name").await?;
    Ok(())
}


async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "invalied").await?;
    Ok(())
}

async fn receive_fullname(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "fullname").await?;
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
