use std::str::FromStr;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{start_again, HandlerResult, MyDialogue};
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
    balance_name: String,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(ref data) = q.data {
        let location: AssetLocation = AssetLocation::from_str(q.data.unwrap().as_str()).unwrap();
        let mut portfolio = Portfolio::get(chat_id.0).unwrap_or(Portfolio::empty());

        portfolio.get_account_mut(&*balance_name).unwrap().set_location(location);
        portfolio.save(chat_id)?;

        start_again(bot, dialogue, chat_id).await?;
    } else {
        todo!()
    }
    Ok(())
}