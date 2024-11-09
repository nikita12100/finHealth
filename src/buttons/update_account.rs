use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{HandlerResult, MyDialogue, State};
use crate::db::portfolio::Portfolio;
use crate::db::db::DataBase;

pub struct UpdateAccountButton;

impl UpdateAccountButton {
    pub const SET_BALANCE: &'static str = "✍️ Установить баланс";
    pub const INCOME_AMOUNT: &'static str = "📈 Внести доход";
    pub const OUTCOME_AMOUNT: &'static str = "📉 Внести расход";

    pub const VALUES: &'static [&'static str; 3] = &[Self::SET_BALANCE, Self::INCOME_AMOUNT, Self::OUTCOME_AMOUNT];
}

pub async fn handler_update_account_btn(bot: Bot, dialogue: MyDialogue, balance_name: String, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    let portfolio = Portfolio::get(q.chat_id().unwrap().0)?;

    match q.data.clone().unwrap().as_str() {
        UpdateAccountButton::SET_BALANCE => {
            let current_balance = portfolio.get_account(&*balance_name).unwrap().get_last_amount().unwrap();
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), format!("Текущее значение {}, укажите новое значение баланса:", current_balance)).await?;

            dialogue.update(State::ListenBalanceAmountFor(balance_name)).await?;
        }
        UpdateAccountButton::INCOME_AMOUNT => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "введите доход:").await?;

            dialogue.update(State::ListenBalanceIncomeFor(balance_name)).await?;
        }
        UpdateAccountButton::OUTCOME_AMOUNT => {
            bot.edit_message_text(chat_id, q.message.clone().unwrap().id(), "введите расход:").await?;

            dialogue.update(State::ListenBalanceOutcomeFor(balance_name)).await?;
        }
        _ => { todo!() }
    }
    Ok(())
}
