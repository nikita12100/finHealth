use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::enums::asset_type::AssetType;
use crate::{goto_start, init_portfolio, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::database::db_account::DataBaseAccount;
use crate::db::database::db_portfolio::DataBasePortfolio;
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
        if let Some(mut portfolio) = Portfolio::get(chat_id.0) {
            let account = portfolio.get_account_mut(&*account_name).unwrap();
            account.set_type(_type.clone());
            account.save(chat_id)?;

            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Тип счета успешно обновлен на {}", _type)).await?;

            goto_start(bot, dialogue, chat_id, None).await?;
        } else {
            log::error!("Portfolio not found for {}", chat_id);
            init_portfolio(chat_id)?;
            let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
            goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
        }
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonType::get_types())).await?;
    }
    Ok(())
}