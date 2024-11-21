use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{goto_start, init_portfolio, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::database::db_account::DataBaseAccount;
use crate::db::database::db_portfolio::DataBasePortfolio;
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
    (account_name, outcome): (String, u32),
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(ref data) = q.data {

        let category: Category = data.as_str().to_owned().parse::<Category>().unwrap();
        if let Some(mut portfolio) = Portfolio::get(q.chat_id().unwrap().0) {
            let account = portfolio.get_account_mut(&*account_name).unwrap();
            account.add_balance_outcome(outcome, category);
            account.save(chat_id)?;

            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Расход успешно сохранен").await?;

            goto_start(bot, dialogue, chat_id, None).await?;
        } else {
            log::error!("Portfolio not found for {}", chat_id);
            init_portfolio(chat_id)?;
            let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
            goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
        }
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonCategory::get_categories())).await?;
    }
    Ok(())
}
