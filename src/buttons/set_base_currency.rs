use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{start_again, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::currency::Currency;

pub struct ButtonBaseCurrency {}
impl ButtonBaseCurrency {
    pub fn get_currencies() -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for c in Currency::iterator() {
            result.push(c.to_string());
        }
        result
    }
}

pub async fn handler_set_base_currency_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    let chat_id =  q.chat_id().unwrap();
    if ButtonBaseCurrency::get_currencies().contains(&q.data.clone().unwrap()) {
        let mut portfolio = Portfolio::get(chat_id.0)?;
        let new_base_currency = Currency::from_str(q.data.unwrap().as_str()).unwrap();
        portfolio.set_base_currency(new_base_currency.clone());
        portfolio.save(chat_id)?;

        bot.send_message(chat_id, format!("Валюта портфеля изменен на {:?}", new_base_currency)).await?;

        start_again(bot, dialogue, chat_id).await?;
        Ok(())
    } else {
        panic!("Incorrect base currency")
    }
}