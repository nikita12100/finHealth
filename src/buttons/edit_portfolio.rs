use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{invalid_input_for_callback, HandlerResult, MyDialogue, State};
use crate::buttons::set_currency::ButtonCurrency;
use crate::utils::common::make_keyboard_string;

pub struct EditPortfolioButton;

impl EditPortfolioButton {
    pub const ADD_BALANCE: &'static str = "Добавить новый баланс";
    pub const SET_BASE_CURRENCY: &'static str = "Установить валюту портфеля";
    pub const SET_EXCHANGE_RATE: &'static str = "Установить курсы валют";

    pub const VALUES: &'static [&'static str; 3] = &[
        Self::ADD_BALANCE,
        Self::SET_BASE_CURRENCY,
        Self::SET_EXCHANGE_RATE,
    ];
}

pub async fn handler_update_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        EditPortfolioButton::ADD_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Давайте добавим новый счет").await?;

            bot.send_message(chat_id, "Напишите как будет новый называться счет:").await?;
            dialogue.update(State::ListenNewAccountName).await?;
        }
        EditPortfolioButton::SET_BASE_CURRENCY => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_BASE_CURRENCY").await?;

            dialogue.update(State::ListenSetBaseCurrencyButtonsCallback).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard_string(1, ButtonCurrency::get_currencies())).await?;
        }
        EditPortfolioButton::SET_EXCHANGE_RATE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_EXCHANGE_RATE").await?;

            bot.send_message(chat_id, "not yet implemented").await?;
            // todo!() // авто получение курса + ручная установка
        }
        _ => {
            invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", EditPortfolioButton::VALUES.to_vec())).await?;
        }
    }
    Ok(())
}
