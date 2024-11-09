use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::buttons::get_portfolio_buttons::GetPortfolioButtons;
use crate::buttons::update_portfolio_buttons::UpdatePortfolioButton;
use crate::db::dao::Portfolio;
use crate::db::db::DataBase;
use crate::utils::mock_data::MockData;

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
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Вы хотите обновить портфель...").await?;
            // dialogue.update(State::ListenUpdatePortfolioButtons).await?;

            let portfolio = Portfolio::get(q.chat_id().unwrap().0)?.get_account_names();
            let mut balances: Vec<&str> = portfolio.iter().map(|x| x as &str).collect();
            balances.extend(UpdatePortfolioButton::VALUES);

            bot.send_message(chat_id, "Выбере какой баланс вы хотите изменить?").reply_markup(make_keyboard(1, balances)).await?;

            dialogue.update(State::ListenBalanceName).await?;
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