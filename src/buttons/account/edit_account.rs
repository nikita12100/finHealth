use std::fmt::format;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{start_again, HandlerResult, MyDialogue, State};
use crate::buttons::account::set_location::ButtonLocation;
use crate::buttons::account::set_type::ButtonType;
use crate::buttons::set_currency::ButtonCurrency;
use crate::db::portfolio::Portfolio;
use crate::db::db::DataBase;
use crate::utils::common::make_keyboard_string;

pub struct EditAccountButton;

impl EditAccountButton {
    pub const SET_BALANCE: &'static str = "✍️ Установить баланс";
    pub const INCOME_AMOUNT: &'static str = "📈 Внести доход";
    pub const OUTCOME_AMOUNT: &'static str = "📉 Внести расход";
    pub const SET_CURRENCY: &'static str = "Изменить валюту счета";
    pub const SET_LOCATION: &'static str = "Изменить ??локацию??";
    pub const SET_TYPE: &'static str = "Изменить тип счета";
    pub const REMOVE_BALANCE: &'static str = "Удалить этот баланс";

    pub const VALUES: &'static [&'static str; 7] = &[
        Self::SET_BALANCE,
        Self::INCOME_AMOUNT,
        Self::OUTCOME_AMOUNT,
        Self::SET_CURRENCY,
        Self::SET_LOCATION,
        Self::SET_TYPE,
        Self::REMOVE_BALANCE,
    ];
}

pub async fn handler_update_account_btn(bot: Bot, dialogue: MyDialogue, balance_name: String, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    let mut portfolio = Portfolio::get(q.chat_id().unwrap().0)?;

    match q.data.clone().unwrap().as_str() {
        EditAccountButton::SET_BALANCE => {
            let current_balance = portfolio.get_account(&*balance_name).unwrap().get_last_amount().unwrap();
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Текущее значение {}, укажите новое значение баланса:", current_balance)).await?;

            dialogue.update(State::ListenBalanceAmountFor(balance_name)).await?;
        }
        EditAccountButton::INCOME_AMOUNT => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "введите доход:").await?;

            dialogue.update(State::ListenBalanceIncomeFor(balance_name)).await?;
        }
        EditAccountButton::OUTCOME_AMOUNT => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "введите расход:").await?;

            dialogue.update(State::ListenBalanceOutcomeFor(balance_name)).await?;
        }
        EditAccountButton::SET_CURRENCY => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_CURRENCY").await?;

            dialogue.update(State::ListenCurrencyFor(balance_name)).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard_string(1, ButtonCurrency::get_currencies())).await?;
        }
        EditAccountButton::SET_LOCATION => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_LOCATION").await?;

            dialogue.update(State::ListenLocationFor(balance_name)).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard_string(1, ButtonLocation::get_locations())).await?;
        }
        EditAccountButton::SET_TYPE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to SET_TYPE").await?;

            dialogue.update(State::ListenTypeFor(balance_name)).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard_string(1, ButtonType::get_types())).await?;
        }
        EditAccountButton::REMOVE_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to REMOVE_BALANCE").await?;
            let account = portfolio.get_account(&*balance_name).unwrap();


            portfolio.delete_account(&account);
            portfolio.save(chat_id)?;
            bot.send_message(chat_id, format!("Баланс {} успешно удален", account.get_name())).await?;
            start_again(bot, dialogue, chat_id).await?;
        }
        _ => { panic!("Error parsing answer") }
    }
    Ok(())
}
