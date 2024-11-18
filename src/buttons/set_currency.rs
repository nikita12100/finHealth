use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{goto_start, init_portfolio, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::currency::Currency;

pub struct ButtonCurrency {}
impl ButtonCurrency {
    pub fn get_currencies() -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for c in Currency::iterator() {
            result.push(c.to_string());
        }
        result
    }
}

pub async fn handler_set_base_currency_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    let chat_id = q.chat_id().unwrap();
    if ButtonCurrency::get_currencies().contains(&q.data.clone().unwrap()) {
        if let Some(mut portfolio) = Portfolio::get(chat_id.0) {
            let new_base_currency = Currency::from_str(q.data.unwrap().as_str()).unwrap();
            portfolio.set_base_currency(new_base_currency.clone());
            portfolio.save(chat_id)?;

            bot.send_message(chat_id, format!("Валюта портфеля изменен на {:?}", new_base_currency)).await?;
            goto_start(bot, dialogue, chat_id, None).await?;
        } else {
        log::error!("Portfolio not found for {}", chat_id);
        init_portfolio(chat_id)?;
        let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
        goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
        }
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCurrency::get_currencies())).await?;
    }
    Ok(())
}

pub async fn handler_set_currency_btn(bot: Bot, dialogue: MyDialogue, account_name: String, q: CallbackQuery) -> HandlerResult {
    let chat_id = q.chat_id().unwrap();
    if ButtonCurrency::get_currencies().contains(&q.data.clone().unwrap()) {
        if let Some(mut portfolio) = Portfolio::get(chat_id.0) {
            let new_base_currency = Currency::from_str(q.data.unwrap().as_str()).unwrap();
            portfolio.get_account_mut(&account_name).unwrap().set_currency(new_base_currency.clone());
            portfolio.save(chat_id)?;

            bot.send_message(chat_id, format!("Валюта счета {} изменен на {:?}", account_name, new_base_currency)).await?;

            goto_start(bot, dialogue, chat_id, None).await?;
        } else {
        log::error!("Portfolio not found for {}", chat_id);
        init_portfolio(chat_id)?;
        let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
        goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
        }
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCurrency::get_currencies())).await?;
    }
    Ok(())
}