mod dao;
mod currency;
mod db;
mod start_buttons;
mod update_portfolio_buttons;
mod mock_data;
mod line_series_example;
mod pie_chart;
mod get_portfolio_buttons;

use std::num::ParseIntError;
use chrono::{TimeZone, Utc};
use teloxide::{
    dispatching::dialogue::{
        serializer::Json,
        ErasedStorage, SqliteStorage, Storage,
    },
    prelude::*,
    utils::command::BotCommands,
};
use crate::dao::{Account, Portfolio};
use rusqlite::Result;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use crate::currency::Currency;
use crate::db::DataBase;
use strum::IntoEnumIterator;
use teloxide::types::PassportElementErrorUnspecifiedType::File;
use crate::get_portfolio_buttons::handler_get_portfolio_btn;
// 0.17.1
use crate::start_buttons::{handler_start_btn, StartButton};
use crate::update_portfolio_buttons::{handler_update_balance_btn, handler_update_portfolio_btn};

type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    /// Listen buttons click
    ListenStartButtons,
    ListenGetPortfolioButtons,
    ListenUpdatePortfolioButtons,
    /// Listen client data from chat
    ListenBalanceName,
    ListenNewBalanceName,
    /// Get client data from chat for each listen
    GotListenBalanceName(String),
    GotNewBalanceName(String),
}

#[derive(Clone, Debug, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "???")]
    Start,
    #[command(description = "???")]
    Help,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting portfolio manager bot...");

    let bot = Bot::from_env();

    Portfolio::create_table().unwrap();

    let storage: MyStorage = SqliteStorage::open("state.sqlite", Json).await.unwrap().erase();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<State>, State>()
                .branch(dptree::case![State::Start].filter_command::<Command>().endpoint(start))
                .branch(dptree::case![State::GotListenBalanceName(String)].endpoint(listen_balance_new_amount))
                .branch(dptree::case![State::ListenNewBalanceName].endpoint(listen_new_balance_name))
                .branch(dptree::case![State::GotNewBalanceName(String)].endpoint(listen_new_balance_amount))
                .branch(dptree::endpoint(invalid_command))
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, ErasedStorage<State>, State>()
                .branch(dptree::case![State::ListenStartButtons].endpoint(handler_start_btn))
                .branch(dptree::case![State::ListenGetPortfolioButtons].endpoint(handler_get_portfolio_btn))
                .branch(dptree::case![State::ListenUpdatePortfolioButtons].endpoint(handler_update_portfolio_btn))
                .branch(dptree::case![State::ListenBalanceName].endpoint(handler_update_balance_btn))
                .endpoint(handler_print),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let intro_text = "Привет, я умею ...\nВыбери действие:";

    dialogue.update(State::ListenStartButtons).await?;
    bot.send_message(msg.chat.id, intro_text).reply_markup(make_keyboard(1, StartButton::VALUES.to_vec())).await?;

    Ok(())
}

async fn start_again(bot: Bot, dialogue: MyDialogue, chat_id: ChatId) -> HandlerResult {
    let intro_text = "Выбери действие:";

    dialogue.update(State::ListenStartButtons).await?;
    bot.send_message(chat_id, intro_text).reply_markup(make_keyboard(1, StartButton::VALUES.to_vec())).await?;

    Ok(())
}

fn make_keyboard(row_size: usize, buttons: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for versions in buttons.chunks(row_size) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

async fn handler_print(bot: Bot, q: CallbackQuery) -> HandlerResult {
    if let Some(ref data) = q.data {
        let text = format!("You chose: \"{data}\"");
        bot.answer_callback_query(&q.id).await?;

        if let Some(message) = q.regular_message() {
            bot.edit_message_text(message.chat.id, message.id, text).await?;
        }
    }

    Ok(())
}

async fn listen_new_balance_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(name) => {
            bot.send_message(msg.chat.id, format!("name will be {:#?}, write please amount:", name)).await?;
            dialogue.update(State::GotNewBalanceName(name.to_string())).await?;
            Ok(())
        }
        None => { todo!() }
    }
}


async fn listen_new_balance_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(amount) => {
            let balance = Account::new(balance_name, amount, Currency::RUB);
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.add_account(balance);
            portfolio.save(msg.chat.id.0)?;

            bot.send_message(msg.chat.id, format!("portfolio saved {:#?}", portfolio)).await?;
            start_again(bot, dialogue, msg.chat.id).await?;

            Ok(())
        }
        Err(_) => { todo!() }
    }
}
async fn listen_balance_new_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(new_balance) => {
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.add_account_record(&*balance_name, new_balance);
            portfolio.save(msg.chat.id.0)?;
            bot.send_message(msg.chat.id, format!("portfolio updated {:#?}", portfolio)).await?;
            start_again(bot, dialogue, msg.chat.id).await?;
        }
        Err(_) => { todo!() }
    }
    Ok(())
}

async fn invalid_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    dialogue.update(State::Start).await?;
    Ok(())
}
