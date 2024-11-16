use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{HandlerResult, MyDialogue, State};
use crate::buttons::set_base_currency::ButtonBaseCurrency;
use crate::utils::common::make_keyboard;

pub struct EditPortfolioButton;

impl EditPortfolioButton {
    pub const ADD_BALANCE: &'static str = "Добавить новый баланс";
    pub const REMOVE_BALANCE: &'static str = "Удалить баланс";
    pub const SET_BASE_CURRENCY: &'static str = "Установить основную валюту";
    pub const SET_EXCHANGE_RATE: &'static str = "Установить курсы валют";

    pub const VALUES: &'static [&'static str; 4] = &[
        Self::ADD_BALANCE,
        Self::REMOVE_BALANCE,
        Self::SET_BASE_CURRENCY,
        Self::SET_EXCHANGE_RATE,
    ];
}

pub async fn handler_update_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        EditPortfolioButton::ADD_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ADD_BALANCE").await?;

            bot.send_message(chat_id, "Напишите как будет называться баланс:").await?;
            dialogue.update(State::ListenNewBalanceName).await?;
        }
        EditPortfolioButton::REMOVE_BALANCE => {
            // bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ResetPortfolio").await?;
            //
            // let empty = Portfolio::empty();
            // empty.save(chat_id.0)?;
            // bot.send_message(chat_id, "Portfolio cleaned, rub_balance.").await?;
            //
            // start_again(bot, dialogue, chat_id).await?;
            todo!()
        }
        EditPortfolioButton::SET_BASE_CURRENCY => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_BASE_CURRENCY").await?;
            let buttons: Vec<String> = ButtonBaseCurrency::get_currencies();
            let buttons_str: Vec<&str> = buttons.iter().map(|s| s.as_str()).collect();

            dialogue.update(State::ListenSetBaseCurrencyButtonsCallback).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard(1, buttons_str)).await?;
        }
        EditPortfolioButton::SET_EXCHANGE_RATE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_EXCHANGE_RATE").await?;
            todo!()
            // let buttons: Vec<String> = ButtonBaseCurrency::get_currencies();
            // let buttons_str: Vec<&str> = buttons.iter().map(|s| s.as_str()).collect();
            //
            // dialogue.update(State::ListenSetBaseCurrencyButtonsCallback).await?;
            // bot.send_message(chat_id, "Выберите для какой валюты будет курс").reply_markup(make_keyboard(1, EditPortfolioButton.to_vec())).await?;
        }
        _ => { todo!() }
    }
    Ok(())
}
