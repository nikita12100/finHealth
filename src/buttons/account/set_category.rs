use crate::db::database::db_account::DataBaseAccount;
use crate::enums::category::Category;
use crate::{
    get_or_create_portfolio, goto_start, invalid_input_for_callback, HandlerResult, MyDialogue,
};
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::Bot;

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
        let mut portfolio = get_or_create_portfolio(chat_id);
        let account = portfolio.get_account_mut(&*account_name).unwrap();
        account.add_balance_outcome(outcome, category);
        account.save(chat_id)?;

        bot.edit_message_text(
            chat_id,
            q.message.clone().unwrap().id(),
            "Расход успешно сохранен",
        )
        .await?;

        goto_start(bot, dialogue, chat_id, None).await?;
    } else {
        invalid_input_for_callback(
            bot,
            dialogue,
            q,
            format!(
                "Необходимо выбрать одну из кнопок {:?}",
                ButtonCategory::get_categories()
            ),
        )
        .await?;
    }
    Ok(())
}
