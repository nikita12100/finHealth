use crate::db::database::db_account::DataBaseAccount;
use crate::enums::asset_location::AssetLocation;
use crate::{
    get_or_create_portfolio, goto_start, invalid_input_for_callback, HandlerResult, MyDialogue,
};
use std::str::FromStr;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::Bot;

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
        let mut portfolio = get_or_create_portfolio(chat_id);

        let account = portfolio.get_account_mut(&*account_name).unwrap();
        account.set_location(location.clone());
        account.save(chat_id)?;

        bot.edit_message_text(
            chat_id,
            q.message.clone().unwrap().id(),
            format!("Локация успешно обновлена на {}", location),
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
                ButtonLocation::get_locations()
            ),
        )
        .await?;
    }
    Ok(())
}
