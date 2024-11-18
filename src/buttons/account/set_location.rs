use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{goto_start, init_portfolio, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;

pub struct ButtonLocation {}
impl ButtonLocation {
    pub fn get_locations() -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for c in AssetLocation::iterator() {
            result.push(c.to_string());
        }
        result
    }
}

pub async fn handler_location_btn(
    bot: Bot,
    dialogue: MyDialogue,
    account_name: String,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(ref data) = q.data {
        let location: AssetLocation = AssetLocation::from_str(data.as_str()).unwrap();
        if let Some(mut portfolio) = Portfolio::get(q.chat_id().unwrap().0) {
            portfolio.get_account_mut(&*account_name).unwrap().set_location(location);
            portfolio.save(chat_id)?;

            goto_start(bot, dialogue, chat_id, None).await?;
        } else {
        log::error!("Portfolio not found for {}", chat_id);
        init_portfolio(chat_id)?;
        let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
        goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
        }
    } else {
        invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", ButtonLocation::get_locations())).await?;
    }
    Ok(())
}