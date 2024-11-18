use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{goto_start, init_portfolio, invalid_input_for_callback, make_keyboard, HandlerResult, MyDialogue, State};
use crate::buttons::get_portfolio::GetPortfolioButtons;
use crate::buttons::update_portfolio::UpdatePortfolioButton;
use crate::db::portfolio::Portfolio;
use crate::db::db::DataBase;
use crate::utils::common::make_keyboard_string;
use crate::utils::mock_data::MockData;

pub struct StartButton;

impl StartButton {
    pub const GET_PORTFOLIO_STATS: &'static str = "📈 Статистика портфеля";
    pub const UPDATE_PORTFOLIO: &'static str = "📝 Обновить портфель";
    pub const HELP: &'static str = "❓ Помощь";

    pub const VALUES: &'static [&'static str; 3] = &[Self::GET_PORTFOLIO_STATS, Self::UPDATE_PORTFOLIO, Self::HELP];
}

pub async fn handler_start_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        StartButton::GET_PORTFOLIO_STATS => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Выберите счет для редактирования").await?;

            dialogue.update(State::ListenGetPortfolioButtonsCallback).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard(1, GetPortfolioButtons::VALUES.to_vec())).await?;
        }
        StartButton::UPDATE_PORTFOLIO => {
            let portfolio_opt = Portfolio::get(chat_id.0);
            let mut accounts = portfolio_opt.map(|p| p.get_account_names()).unwrap_or(Vec::new());
            if !accounts.is_empty() {
                bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Вы хотите обновить портфель").await?;
                let buttons: Vec<String> = UpdatePortfolioButton::VALUES.iter().map(|s| s.to_string()).collect();
                accounts.extend(buttons);

                bot.send_message(chat_id, "Выберите какой баланс вы хотите изменить:").reply_markup(make_keyboard_string(1, accounts)).await?;
                dialogue.update(State::ListenBalanceNameCallback).await?;
            } else {
                init_portfolio(chat_id)?;
                bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "У вас нет баланса, давайте добавим первый").await?;
                bot.send_message(chat_id, "Напишите как будет новый называться счет:").await?;
                dialogue.update(State::ListenNewAccountName).await?;
            }
        }
        StartButton::HELP => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to HELP").await?;
            // todo тут нежно сообщение
            MockData::create().save(chat_id)?; // todo dev

            goto_start(bot, dialogue, chat_id, None).await?;
        }
        _ => {
            invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", StartButton::VALUES.to_vec())).await?;
        }
    }
    Ok(())
}