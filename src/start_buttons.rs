use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::db::DataBase;
use crate::get_portfolio_buttons::GetPortfolioButtons;
use crate::mock_data::MockData;
use crate::update_portfolio_buttons::UpdatePortfolioButton;

pub struct StartButton;

impl StartButton {
    pub const GET_PORTFOLIO_STATS: &'static str = "Статистика портфеля";
    pub const UPDATE_PORTFOLIO: &'static str = "Обновить портфель";
    pub const HELP: &'static str = "Помощь";

    pub const VALUES: &'static [&'static str; 3] = &[Self::GET_PORTFOLIO_STATS, Self::UPDATE_PORTFOLIO, Self::HELP];
}

pub async fn handler_start_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        StartButton::GET_PORTFOLIO_STATS => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to GET_PORTFOLIO_STATS").await?;

            dialogue.update(State::ListenGetPortfolioButtons).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard(1, GetPortfolioButtons::VALUES.to_vec())).await?;
        }
        StartButton::UPDATE_PORTFOLIO => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to UPDATE_PORTFOLIO").await?;

            dialogue.update(State::ListenUpdatePortfolioButtons).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard(1, UpdatePortfolioButton::VALUES.to_vec())).await?;
        }
        StartButton::HELP => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to HELP").await?;

            MockData::create().save(chat_id.0)?;

            start_again(bot, dialogue, chat_id).await?;
        }
        _ => { todo!() }
    }
    Ok(())
}