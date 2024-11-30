use crate::buttons::get_portfolio::GetPortfolioButtons;
use crate::buttons::update_portfolio::UpdatePortfolioButton;
use crate::db::database::db_portfolio::DataBasePortfolio;
use crate::db::portfolio::Portfolio;
use crate::utils::common::make_keyboard_string;
use crate::utils::text_const::HELP_MESSAGE;
use crate::{
    get_or_create_portfolio, goto_start, invalid_input_for_callback, make_keyboard, HandlerResult,
    MyDialogue, State,
};
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::Bot;

pub struct StartButton;

impl StartButton {
    pub const GET_PORTFOLIO_STATS: &'static str = "📈 Статистика портфеля";
    pub const UPDATE_PORTFOLIO: &'static str = "📝 Обновить портфель";
    pub const HELP: &'static str = "❓ Помощь";

    pub const VALUES: &'static [&'static str; 3] = &[
        Self::GET_PORTFOLIO_STATS,
        Self::UPDATE_PORTFOLIO,
        Self::HELP,
    ];
}

pub async fn handler_start_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        StartButton::GET_PORTFOLIO_STATS => {
            bot.edit_message_text(
                chat_id,
                q.message.clone().unwrap().id(),
                "Выберите счет для редактирования",
            )
            .await?;

            dialogue
                .update(State::ListenGetPortfolioButtonsCallback)
                .await?;
            bot.send_message(chat_id, "Выберите статистику")
                .reply_markup(make_keyboard(1, GetPortfolioButtons::VALUES.to_vec()))
                .await?;
        }
        StartButton::UPDATE_PORTFOLIO => {
            let portfolio_opt = Portfolio::get(chat_id.0);
            let mut accounts = portfolio_opt
                .map(|p| p.get_account_names())
                .unwrap_or(Vec::new());
            if !accounts.is_empty() {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Вы хотите обновить портфель",
                )
                .await?;
                let buttons: Vec<String> = UpdatePortfolioButton::VALUES
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                accounts.extend(buttons);

                bot.send_message(chat_id, "Выберите какой баланс вы хотите изменить:")
                    .reply_markup(make_keyboard_string(1, accounts))
                    .await?;
                dialogue
                    .update(State::ListenBalanceNameUpdateBalanceCallback)
                    .await?;
            } else {
                get_or_create_portfolio(chat_id);
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "У вас нет баланса, давайте добавим первый",
                )
                .await?;
                bot.send_message(chat_id, "Напишите как будет новый называться счет:")
                    .await?;
                dialogue.update(State::ListenNewAccountName).await?;
            }
        }
        StartButton::HELP => {
            bot.edit_message_text(
                chat_id,
                q.message.clone().unwrap().id(),
                "Как работать с ботом:",
            )
            .await?;

            goto_start(bot, dialogue, chat_id, Some(HELP_MESSAGE.to_string())).await?;
        }
        _ => {
            invalid_input_for_callback(
                bot,
                dialogue,
                q,
                format!(
                    "Необходимо выбрать одну из кнопок {:?}",
                    StartButton::VALUES.to_vec()
                ),
            )
            .await?;
        }
    }
    Ok(())
}
