mod buttons;
mod charts;
mod db;
mod utils;
mod enums;

use teloxide::{
    dispatching::dialogue::{
        serializer::Json,
        ErasedStorage, SqliteStorage, Storage,
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
use crate::db::account::Account;
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::category::Category;
use crate::enums::currency::Currency;
use crate::utils::common::{make_keyboard, make_keyboard_string};

type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    // Listen buttons click
    ListenStartButtonsCallback,
    ListenGetPortfolioButtonsCallback,
    ListenEditPortfolioButtonsCallback,
    ListenSetBaseCurrencyButtonsCallback,
    ListenCategoryCallback {
        balance_name: String,
        outcome: u32,
    },
    // Listen client data from chat
    ListenBalanceName,
    ListenNewBalanceName,
    ListenBalanceAmountFor(String),
    ListenBalanceIncomeFor(String),
    ListenBalanceOutcomeFor(String),
    ListenCurrencyFor(String),
    ListenLocationFor(String),
    ListenTypeFor(String),
    // Get client data from chat for each listen
    GotListenBalanceNameListenAccountButtons(String),
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
    std::env::set_var("RUST_LOG", "info");
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
                .branch(dptree::case![State::ListenNewBalanceName].endpoint(listen_new_balance_name))
                .branch(dptree::case![State::GotNewBalanceName(new_balance_name)].endpoint(listen_new_balance_amount))
                .branch(dptree::case![State::ListenBalanceAmountFor(balance_name)].endpoint(listen_balance_new_amount))
                .branch(dptree::case![State::ListenBalanceIncomeFor(balance_name)].endpoint(listen_balance_income_amount))
                .branch(dptree::case![State::ListenBalanceOutcomeFor(balance_name)].endpoint(listen_balance_outcome_amount))
                .branch(dptree::endpoint(invalid_command))
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, ErasedStorage<State>, State>()
                .branch(dptree::case![State::ListenStartButtonsCallback].endpoint(handler_start_btn))
                .branch(dptree::case![State::ListenGetPortfolioButtonsCallback].endpoint(handler_get_portfolio_btn))
                .branch(dptree::case![State::ListenEditPortfolioButtonsCallback].endpoint(handler_update_portfolio_btn))
                .branch(dptree::case![State::ListenCurrencyFor(balance_name)].endpoint(handler_set_currency_btn))
                .branch(dptree::case![State::ListenLocationFor(balance_name)].endpoint(handler_location_btn))
                .branch(dptree::case![State::ListenTypeFor(balance_name)].endpoint(handler_type_btn))
                .branch(dptree::case![State::ListenSetBaseCurrencyButtonsCallback].endpoint(handler_set_base_currency_btn))
                .branch(dptree::case![State::ListenCategoryCallback{balance_name, outcome}].endpoint(handler_category_btn))
                .branch(dptree::case![State::ListenBalanceName].endpoint(handler_update_balance_btn))
                .branch(dptree::case![State::GotListenBalanceNameListenAccountButtons(balance_name)].endpoint(handler_update_account_btn))

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

    dialogue.update(State::ListenStartButtonsCallback).await?;
    bot.send_message(msg.chat.id, intro_text).reply_markup(make_keyboard(1, StartButton::VALUES.to_vec())).await?;

    Ok(())
}

async fn start_again(bot: Bot, dialogue: MyDialogue, chat_id: ChatId) -> HandlerResult {
    let intro_text = "Выбери действие:";

    dialogue.update(State::ListenStartButtonsCallback).await?;
    bot.send_message(chat_id, intro_text).reply_markup(make_keyboard(1, StartButton::VALUES.to_vec())).await?;

    Ok(())
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
        None => { panic!("Error parsing answer") }
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
            let balance = Account::new(balance_name, amount, Currency::Rub, AssetLocation::Other, AssetType::default());
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.add_account(balance);
            portfolio.save(msg.chat.id)?;

            bot.send_message(msg.chat.id, format!("portfolio saved {:#?}", portfolio)).await?;
            start_again(bot, dialogue, msg.chat.id).await?;

            Ok(())
        }
        Err(_) => { panic!("Error parsing answer") }
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

            portfolio.get_account_mut(&*balance_name).unwrap().set_balance_amount(new_balance, None);
            portfolio.save(msg.chat.id)?;
            bot.send_message(msg.chat.id, format!("portfolio updated {:#?}", portfolio)).await?;
            start_again(bot, dialogue, msg.chat.id).await?;
        }
        Err(_) => { panic!("Error parsing answer") }
    }
    Ok(())
}

async fn listen_balance_income_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(income) => {
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.get_account_mut(&*balance_name).unwrap().add_balance_income(income);
            portfolio.save(msg.chat.id)?;
            bot.send_message(msg.chat.id, format!("portfolio updated {:#?}", portfolio)).await?;
            start_again(bot, dialogue, msg.chat.id).await?;
        }
        Err(_) => { panic!("Error parsing answer") }
    }
    Ok(())
}

async fn listen_balance_outcome_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(outcome) => {
            dialogue.update(State::ListenCategoryCallback { balance_name, outcome }).await?;
            let buttons: Vec<String> = Category::iterator().map(|c| c.to_string()).collect();

            bot.send_message(msg.chat.id, "Выберите категорию трат:").reply_markup(make_keyboard_string(3, buttons)).await?;
        }
        Err(_) => { panic!("Error parsing answer") }
    }
    Ok(())
}

async fn invalid_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    dialogue.update(State::Start).await?;
    Ok(())
}
