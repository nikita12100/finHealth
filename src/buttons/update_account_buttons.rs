use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::db::dao::Portfolio;
use crate::db::db::DataBase;

pub struct UpdateAccountButton;

impl UpdateAccountButton {
    pub const SET_BALANCE: &'static str = "Установить баланс";
    pub const INCOME_AMOUNT: &'static str = "Внести доход";
    pub const OUTCOME_AMOUNT: &'static str = "Внести расход";
    pub const CHANGE_CURRENCY: &'static str = "Изменить валюту";

    pub const VALUES: &'static [&'static str; 4] = &[Self::SET_BALANCE, Self::INCOME_AMOUNT, Self::OUTCOME_AMOUNT, Self::CHANGE_CURRENCY];
}

pub async fn handler_update_account_btn(bot: Bot, dialogue: MyDialogue, balance_name: String, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        UpdateAccountButton::SET_BALANCE => {
            todo!()
        }
        UpdateAccountButton::INCOME_AMOUNT => {
            todo!()
        }
        UpdateAccountButton::OUTCOME_AMOUNT => {
            todo!()
        }
        UpdateAccountButton::CHANGE_CURRENCY => {
            todo!()
        }
        _ => { todo!() }
    }
    Ok(())
}
