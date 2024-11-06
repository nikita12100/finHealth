use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::dao::Portfolio;
use crate::db::DataBase;

pub struct UpdatePortfolioButton;

impl UpdatePortfolioButton { /// обновить портфель -> [показать все балансы], "редактировать балансы"
    pub const UPDATE_BALANCE: &'static str = "Обновить баланс"; /// сделать кнопки для доход/расход
    pub const ADD_BALANCE: &'static str = "Добавить новый баланс";
    pub const REMOVE_BALANCE: &'static str = "Удалить баланс";

    pub const VALUES: &'static [&'static str; 3] = &[Self::UPDATE_BALANCE, Self::ADD_BALANCE, Self::REMOVE_BALANCE];
}

pub async fn handler_update_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        UpdatePortfolioButton::UPDATE_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to UPDATE_BALANCE").await?;

            let p = Portfolio::get(q.chat_id().unwrap().0)?.get_account_names();
            let balances = p.iter().map(|x| x as &str).collect();
            bot.send_message(chat_id, "Какой баланс изменить?").reply_markup(make_keyboard(1, balances)).await?;

            dialogue.update(State::ListenBalanceName).await?;
        }
        UpdatePortfolioButton::ADD_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ADD_BALANCE").await?;

            bot.send_message(chat_id, "Напишите как будет называться баланс:").await?;
            dialogue.update(State::ListenNewBalanceName).await?;
        }
        UpdatePortfolioButton::REMOVE_BALANCE => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ResetPortfolio").await?;

            let empty = Portfolio::empty();
            empty.save(chat_id.0)?;
            bot.send_message(chat_id, "Portfolio cleaned, rub_balance.").await?;

            start_again(bot, dialogue, chat_id).await?;
        }
        _ => { todo!() }
    }
    Ok(())
}

pub async fn handler_update_balance_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    let chosen_balance = q.data.unwrap();

    bot.send_message(chat_id, format!("Вы хотите изменить {:?}, введите новое значение:", chosen_balance)).await?;
    dialogue.update(State::GotListenBalanceName(chosen_balance)).await?;

    Ok(())
}