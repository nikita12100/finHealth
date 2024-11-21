use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, HandlerResult, MyDialogue, State};
use crate::buttons::account::edit_account::EditAccountButton;
use crate::buttons::edit_portfolio::EditPortfolioButton;
use crate::db::database::db_portfolio::DataBasePortfolio;
use crate::db::portfolio::Portfolio;

pub struct UpdatePortfolioButton;

impl UpdatePortfolioButton {
    pub const EDIT_BALANCES: &'static str = "Редактировать портфель";

    pub const VALUES: &'static [&'static str; 1] = &[
        Self::EDIT_BALANCES,
    ];
}

pub async fn handler_update_balance_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        UpdatePortfolioButton::EDIT_BALANCES => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "Как вы хотите изменить портфель?").await?;

            dialogue.update(State::ListenEditPortfolioButtonsCallback).await?;
            bot.send_message(chat_id, "Выберите что изменить:").reply_markup(make_keyboard(1, EditPortfolioButton::VALUES.to_vec())).await?;
        }
        _ => {
            if let Some(portfolio) = Portfolio::get(q.chat_id().unwrap().0) {
                let balances = portfolio.get_account_names();
                let chosen_balance = q.data.unwrap();
                assert!(balances.contains(&chosen_balance));

                bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Редактирование счета {}", chosen_balance)).await?;
                bot.send_message(chat_id, "Выберете действие:")
                    .reply_markup(make_keyboard(1, EditAccountButton::VALUES.to_vec())).await.unwrap();

                dialogue.update(State::GotListenAccountNameListenAccountButtonsCallback(chosen_balance)).await?;
            } else {
                bot.send_message(chat_id, "У вас еще нет баланса, введите имя для нового баланса:").await?;
                dialogue.update(State::ListenNewAccountName).await?;
            }
        }
    }

    Ok(())
}