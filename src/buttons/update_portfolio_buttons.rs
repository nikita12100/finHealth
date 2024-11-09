use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::buttons::update_account_buttons::UpdateAccountButton;
use crate::db::dao::Portfolio;
use crate::db::db::DataBase;

pub struct UpdatePortfolioButton;

impl UpdatePortfolioButton {
    pub const REMOVE_BALANCE: &'static str = "Редактировать балансы";

    pub const VALUES: &'static [&'static str; 1] = &[Self::REMOVE_BALANCE];
}

pub async fn handler_update_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        // UpdatePortfolioButton::UPDATE_BALANCE => {
        //     bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to UPDATE_BALANCE").await?;
        //
        //     let p = Portfolio::get(q.chat_id().unwrap().0)?.get_account_names();
        //     let balances = p.iter().map(|x| x as &str).collect();
        //     bot.send_message(chat_id, "Какой баланс изменить?").reply_markup(make_keyboard(1, balances)).await?;
        //
        //     dialogue.update(State::ListenBalanceName).await?;
        // }
        // UpdatePortfolioButton::ADD_BALANCE => {
        //     bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to ADD_BALANCE").await?;
        //
        //     bot.send_message(chat_id, "Напишите как будет называться баланс:").await?;
        //     dialogue.update(State::ListenNewBalanceName).await?;
        // }
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

    match q.data.clone().unwrap().as_str() {
        UpdatePortfolioButton::REMOVE_BALANCE => {
            todo!()
        }
        _ => {
            let portfolio = Portfolio::get(q.chat_id().unwrap().0)?.get_account_names();
            let balances: Vec<&str> = portfolio.iter().map(|x| x as &str).collect();

            let chosen_balance = q.data.unwrap();
            assert!(balances.contains(&&*chosen_balance));

            bot.send_message(chat_id, format!("Вы хотите изменить {:?}, выберете действие:", chosen_balance))
                .reply_markup(make_keyboard(1, UpdateAccountButton::VALUES.to_vec())).await?;
            dialogue.update(State::GotListenBalanceNameListenAccountButtons(chosen_balance)).await?;
        }
    }

    Ok(())
}