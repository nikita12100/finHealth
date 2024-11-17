use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{start_again, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::category::Category;

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

        start_again(bot, dialogue, chat_id).await?;
    } else {
        panic!("Error parsing answer")
    }
    Ok(())
}
