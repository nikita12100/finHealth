use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::db::dao::Portfolio;
use crate::db::db::DataBase;

pub struct EditPortfolioButton;

impl EditPortfolioButton {
    pub const ADD_BALANCE: &'static str = "Добавить новый баланс";
    pub const REMOVE_BALANCE: &'static str = "Удалить баланс";

    pub const VALUES: &'static [&'static str; 2] = &[Self::ADD_BALANCE, Self::REMOVE_BALANCE];
}

pub async fn handler_update_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        EditPortfolioButton::ADD_BALANCE => {
            todo!()
        }
        EditPortfolioButton::REMOVE_BALANCE => {
            todo!()
        }
        _ => { todo!() }
    }
    Ok(())
}
