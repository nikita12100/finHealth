use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{get_or_create_portfolio, goto_start, invalid_input_for_callback, HandlerResult, MyDialogue, State};
use crate::buttons::account::set_location::ButtonLocation;
use crate::buttons::account::set_type::ButtonType;
use crate::buttons::set_currency::ButtonCurrency;
use crate::db::database::db_account::DataBaseAccount;
use crate::utils::common::make_keyboard_string;

pub struct EditAccountButton;

impl EditAccountButton {
    pub const INCOME_AMOUNT: &'static str = "📈 Внести доход";
    pub const OUTCOME_AMOUNT: &'static str = "📉 Внести расход";
    pub const SET_CURRENCY: &'static str = "Изменить валюту счета";
    pub const SET_LOCATION: &'static str = "Изменить ??локацию??";
    pub const SET_TYPE: &'static str = "Изменить тип счета";
    pub const REMOVE_BALANCE: &'static str = "Удалить этот баланс";

    pub const VALUES: &'static [&'static str; 6] = &[
        Self::INCOME_AMOUNT,
        Self::OUTCOME_AMOUNT,
        Self::SET_CURRENCY,
        Self::SET_LOCATION,
        Self::SET_TYPE,
        Self::REMOVE_BALANCE,
    ];
}

pub async fn handler_update_account_btn(bot: Bot, dialogue: MyDialogue, account_name: String, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    let portfolio = get_or_create_portfolio(chat_id);
    match q.data.clone().unwrap().as_str() {
        EditAccountButton::INCOME_AMOUNT => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Введите сумму дохода:").await?;

            dialogue.update(State::ListenAccountIncomeFor(account_name)).await?;
        }
        EditAccountButton::OUTCOME_AMOUNT => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Введите сумму расхода:").await?;

            dialogue.update(State::ListenAccountOutcomeFor(account_name)).await?;
        }
        EditAccountButton::SET_CURRENCY => {
            let current_currency = portfolio.get_account(&*account_name).unwrap().get_currency().clone();
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Текущая валюта счета {}", current_currency.to_string())).await?;

            dialogue.update(State::ListenCurrencyForAccountCallback(account_name)).await?;
            bot.send_message(chat_id, "Выберите валюту").reply_markup(make_keyboard_string(1, ButtonCurrency::get_currencies())).await?;
        }
        EditAccountButton::SET_LOCATION => {
            let current_location = portfolio.get_account(&*account_name).unwrap().get_location().clone();
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Текущая локация счета {}", current_location.to_string())).await?;

            dialogue.update(State::ListenLocationForCallback(account_name)).await?;
            bot.send_message(chat_id, "Выберите локацию").reply_markup(make_keyboard_string(1, ButtonLocation::get_locations())).await?;
        }
        EditAccountButton::SET_TYPE => {
            let current_type = portfolio.get_account(&*account_name).unwrap().get_type().clone();

            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Текущий тип счета {}", current_type.to_string())).await?;

            dialogue.update(State::ListenTypeForCallback(account_name)).await?;
            bot.send_message(chat_id, "Выберите тип").reply_markup(make_keyboard_string(1, ButtonType::get_types())).await?;
        }
        EditAccountButton::REMOVE_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Вы хотите удалить счет").await?;
            let account = portfolio.get_account(&*account_name).unwrap();
            account.delete()?;

            bot.send_message(chat_id, format!("Баланс {} успешно удален", account.get_name())).await?;
            goto_start(bot, dialogue, chat_id, None).await?;
        }
        _ => {
            invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", EditAccountButton::VALUES.to_vec())).await?;
        }
    }

    Ok(())
}
