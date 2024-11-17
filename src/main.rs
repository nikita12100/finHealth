mod buttons;
mod charts;
mod db;
mod utils;
mod enums;
mod listeners_input;

use teloxide::{
    dispatching::dialogue::{
        serializer::Json,
        ErasedStorage, SqliteStorage, Storage,
        GetChatId,
    },
    prelude::*,
    utils::command::BotCommands,
};
use rusqlite::Result;
use crate::buttons::account::edit_account::handler_update_account_btn;
use crate::buttons::account::set_category::handler_category_btn;
use crate::buttons::account::set_location::handler_location_btn;
use crate::buttons::account::set_type::handler_type_btn;
use crate::buttons::edit_portfolio::handler_update_portfolio_btn;
use crate::buttons::get_portfolio::handler_get_portfolio_btn;
use crate::buttons::set_currency::{handler_set_base_currency_btn, handler_set_currency_btn};
use crate::buttons::start::{handler_start_btn, StartButton};
use crate::buttons::update_portfolio::handler_update_balance_btn;
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::listeners_input::*;
use crate::utils::common::make_keyboard;
use crate::utils::text_const::{INVALID_COMMAND_TEXT, UNKNOWN_ERROR};

type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State { // todo replace in enums
    #[default]
    Start,
    // Listen buttons click
    ListenStartButtonsCallback,
    ListenGetPortfolioButtonsCallback,
    ListenEditPortfolioButtonsCallback,
    ListenSetBaseCurrencyButtonsCallback,
    ListenCategoryCallback {
        account_name: String,
        outcome: u32,
    },
    // Listen client data from chat
    ListenBalanceNameCallback,
    ListenNewAccountName,
    ListenAccountAmountFor(String),
    ListenAccountIncomeFor(String),
    ListenAccountOutcomeFor(String),
    ListenCurrencyForCallback(String),
    ListenLocationForCallback(String),
    ListenTypeForCallback(String),
    // Get client data from chat for each listen
    GotListenAccountNameListenAccountButtonsCallback(String),
    GotNewAccountName(String),
}

#[derive(Clone, Debug, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "В начало")]
    Start,
    #[command(description = "Показать все команды")]
    Help,
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Starting portfolio manager bot...");

    let bot = Bot::from_env();

    Portfolio::create_tables().unwrap();

    let storage: MyStorage = SqliteStorage::open("state.sqlite", Json).await.unwrap().erase();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<State>, State>()
                .branch(dptree::case![State::Start].filter_command::<Command>().endpoint(start))
                .branch(dptree::case![State::ListenNewAccountName].endpoint(listen_new_account_name))
                .branch(dptree::case![State::GotNewAccountName(new_account_name)].endpoint(listen_new_account_amount))
                .branch(dptree::case![State::ListenAccountAmountFor(account_name)].endpoint(listen_account_new_amount))
                .branch(dptree::case![State::ListenAccountIncomeFor(account_name)].endpoint(listen_account_income_amount))
                .branch(dptree::case![State::ListenAccountOutcomeFor(account_name)].endpoint(listen_account_outcome_amount))
                .branch(dptree::endpoint(|b, d, m: Message| goto_start(b, d, m.chat.id, Some(INVALID_COMMAND_TEXT.to_string()))))
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, ErasedStorage<State>, State>()
                .branch(dptree::case![State::ListenStartButtonsCallback].endpoint(handler_start_btn))
                .branch(dptree::case![State::ListenGetPortfolioButtonsCallback].endpoint(handler_get_portfolio_btn))
                .branch(dptree::case![State::ListenEditPortfolioButtonsCallback].endpoint(handler_update_portfolio_btn))
                .branch(dptree::case![State::ListenCurrencyForCallback(account_name)].endpoint(handler_set_currency_btn))
                .branch(dptree::case![State::ListenLocationForCallback(account_name)].endpoint(handler_location_btn))
                .branch(dptree::case![State::ListenTypeForCallback(account_name)].endpoint(handler_type_btn))
                .branch(dptree::case![State::ListenSetBaseCurrencyButtonsCallback].endpoint(handler_set_base_currency_btn))
                .branch(dptree::case![State::ListenCategoryCallback{account_name, outcome}].endpoint(handler_category_btn))
                .branch(dptree::case![State::ListenBalanceNameCallback].endpoint(handler_update_balance_btn))
                .branch(dptree::case![State::GotListenAccountNameListenAccountButtonsCallback(account_name)].endpoint(handler_update_account_btn))
                .endpoint(|b, d, q| invalid_input_for_callback(b, d, q, UNKNOWN_ERROR.to_string())),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let intro_text = "Привет, я умею ...\nВыберите действие:";

    dialogue.update(State::ListenStartButtonsCallback).await?;
    bot.send_message(msg.chat.id, intro_text).reply_markup(make_keyboard(1, StartButton::VALUES.to_vec())).await?;

    Ok(())
}

async fn goto_start(bot: Bot, dialogue: MyDialogue, chat_id: ChatId, error_text: Option<String>) -> HandlerResult {
    if let Some(text) = error_text {
        bot.send_message(chat_id, text).await?;
    }

    let intro_text = "Выберите действие:";
    dialogue.update(State::ListenStartButtonsCallback).await?;
    bot.send_message(chat_id, intro_text).reply_markup(make_keyboard(1, StartButton::VALUES.to_vec())).await?;

    Ok(())
}

async fn invalid_input_for_callback(bot: Bot, dialogue: MyDialogue, q: CallbackQuery, text: String) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    goto_start(bot, dialogue, q.chat_id().unwrap(), Some(text)).await?;

    Ok(())
}
