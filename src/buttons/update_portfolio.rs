use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{make_keyboard, HandlerResult, MyDialogue, State};
use crate::buttons::edit_portfolio::EditPortfolioButton;
use crate::buttons::update_account::UpdateAccountButton;
use crate::db::portfolio::Portfolio;
use crate::db::db::DataBase;

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
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "you want to EDIT_BALANCES").await?;

            dialogue.update(State::ListenEditPortfolioButtonsCallback).await?;
            bot.send_message(chat_id, "Chose").reply_markup(make_keyboard(1, EditPortfolioButton::VALUES.to_vec())).await?;
        }
        _ => {
            let portfolio_opt = Portfolio::get(q.chat_id().unwrap().0);
            match portfolio_opt {
                Err(_) => {
                    bot.send_message(chat_id, "У вас еще нет баланса, введите имя для нового баланса:").await?;
                    dialogue.update(State::ListenNewBalanceName).await?;
                }
                Ok(portfolio) => {
                    let balances = portfolio.get_account_names();
                    let chosen_balance = q.data.unwrap();
                    assert!(balances.contains(&chosen_balance));

                    bot.send_message(chat_id, format!("Вы хотите изменить {:?}, выберете действие:", chosen_balance))
                        .reply_markup(make_keyboard(1, UpdateAccountButton::VALUES.to_vec())).await?;
                    dialogue.update(State::GotListenBalanceNameListenAccountButtons(chosen_balance)).await?;
                }
            }
        }
    }

    Ok(())
}