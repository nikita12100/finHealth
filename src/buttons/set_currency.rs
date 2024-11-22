use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{get_or_create_portfolio, goto_start, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::database::db_account::DataBaseAccount;
use crate::db::database::db_portfolio::DataBasePortfolio;
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
        let mut portfolio = get_or_create_portfolio(chat_id);
        let new_base_currency: Currency = Currency::from_str(q.data.unwrap().as_str()).unwrap();
        portfolio.set_base_currency(new_base_currency.clone());
        portfolio.save(chat_id)?;

        bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Валюта портфеля изменена на {}", new_base_currency.to_string())).await?;
        goto_start(bot, dialogue, chat_id, None).await?;
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCurrency::get_currencies())).await?;
    }
    Ok(())
}

pub async fn handler_set_currency_btn(bot: Bot, dialogue: MyDialogue, account_name: String, q: CallbackQuery) -> HandlerResult {
    let chat_id = q.chat_id().unwrap();
    if ButtonCurrency::get_currencies().contains(&q.data.clone().unwrap()) {
        let mut portfolio = get_or_create_portfolio(chat_id);
        let new_base_currency = Currency::from_str(q.data.unwrap().as_str()).unwrap();
        let account = portfolio.get_account_mut(&account_name).unwrap();
        account.set_currency(new_base_currency.clone());
        account.save(chat_id)?;

        bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Валюта счета {} изменена на {}", account_name, new_base_currency)).await?;

        goto_start(bot, dialogue, chat_id, None).await?;
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCurrency::get_currencies())).await?;
    }
    Ok(())
}

// pub async fn handler_set_currency_exchange_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
//     let chat_id = q.chat_id().unwrap();
//     if ButtonCurrency::get_currencies().contains(&q.data.clone().unwrap()) {
//         let mut portfolio = get_or_create_portfolio(chat_id);
//         let currency = Currency::from_str(q.data.unwrap().as_str()).unwrap();
//
//         portfolio.get_exchange_rate()
//
//         let account = portfolio.get_account_mut(&account_name).unwrap();
//         account.set_currency(new_base_currency.clone());
//         account.save(chat_id)?;
//
//         bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Валюта счета {} изменена на {}", account_name, new_base_currency)).await?;
//
//         goto_start(bot, dialogue, chat_id, None).await?;
//     } else {
//         invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCurrency::get_currencies())).await?;
//     }
//     Ok(())
// }