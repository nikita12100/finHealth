use crate::db::database::db_account::DataBaseAccount;
use crate::enums::asset_type::AssetType;
use crate::{
    get_or_create_portfolio, goto_start, invalid_input_for_callback, HandlerResult, MyDialogue,
};
use std::str::FromStr;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::Bot;

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
        let mut portfolio = get_or_create_portfolio(chat_id);

        let account = portfolio.get_account_mut(&*account_name).unwrap();
        account.set_type(_type.clone());
        account.save(chat_id)?;

        bot.edit_message_text(
            chat_id,
            q.message.clone().unwrap().id(),
            format!("Тип счета успешно обновлен на {}", _type),
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
                ButtonType::get_types()
            ),
        )
        .await?;
    }
    Ok(())
}
