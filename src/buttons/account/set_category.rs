use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{goto_start, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::category::Category;

pub struct ButtonCategory {}

impl ButtonCategory {
    pub fn get_categories() -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for c in Category::iterator() {
            result.push(c.to_string());
        }
        result
    }
}

pub async fn handler_category_btn(
    bot: Bot,
    dialogue: MyDialogue,
    (balance_name, outcome): (String, u32),
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(ref data) = q.data {
        let category_string :String = data.as_str().chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect::<String>().trim().to_string();
        let category: Category = Category::from_str(&category_string).unwrap();
        let mut portfolio = Portfolio::get(chat_id.0).unwrap_or(Portfolio::empty());

        portfolio.get_account_mut(&*balance_name).unwrap().add_balance_outcome(outcome, category);
        portfolio.save(chat_id)?;

        goto_start(bot, dialogue, chat_id).await?;
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCategory::get_categories())).await?;
    }
    Ok(())
}
