use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::enums::asset_type::AssetType;
use crate::{goto_start, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;

pub struct ButtonType {}

impl ButtonType {
    pub fn get_types() -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for c in AssetType::iterator() {
            result.push(c.to_string());
        }
        result
    }
}

pub async fn handler_type_btn(
    bot: Bot,
    dialogue: MyDialogue,
    account_name: String,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(ref data) = q.data {
        let _type: AssetType = AssetType::from_str(data.as_str()).unwrap();
        let mut portfolio = Portfolio::get(chat_id.0).unwrap_or(Portfolio::empty());

        portfolio.get_account_mut(&*account_name).unwrap().set_type(_type);
        portfolio.save(chat_id)?;

        goto_start(bot, dialogue, chat_id, None).await?;
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonType::get_types())).await?;
    }
    Ok(())
}