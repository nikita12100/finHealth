use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{HandlerResult, MyDialogue, State};

pub struct EditPortfolioButton;

impl EditPortfolioButton {
    pub const ADD_BALANCE: &'static str = "Добавить новый баланс";
    pub const REMOVE_BALANCE: &'static str = "Удалить баланс";
    // pub const CHANGE_CURRENCY: &'static str = "Изменить валюту баланса";

    pub const VALUES: &'static [&'static str; 2] = &[Self::ADD_BALANCE, Self::REMOVE_BALANCE];
}

pub async fn handler_update_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        EditPortfolioButton::ADD_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ADD_BALANCE").await?;

            bot.send_message(chat_id, "Напишите как будет называться баланс:").await?;
            dialogue.update(State::ListenNewBalanceName).await?;
        }
        EditPortfolioButton::REMOVE_BALANCE => {
            // bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ResetPortfolio").await?;
            //
            // let empty = Portfolio::empty();
            // empty.save(chat_id.0)?;
            // bot.send_message(chat_id, "Portfolio cleaned, rub_balance.").await?;
            //
            // start_again(bot, dialogue, chat_id).await?;
            todo!()
        }
        _ => { todo!() }
    }
    Ok(())
}
