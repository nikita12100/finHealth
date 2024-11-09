use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{start_again, HandlerResult, MyDialogue};
use crate::db::portfolio::Portfolio;
use crate::db::db::DataBase;

pub struct GetPortfolioButtons {}

impl GetPortfolioButtons {
    pub const DRAW_CURRENT_ALLOCATIONS: &'static str = "Показать текущие вложения";
    // pub const DRAW_CURRENT_ALLOCATIONS: &'static str = "Показать траты за все время по балансу";
    pub const RAW_BALANCE: &'static str = "[DEV] Показать сырой баланс";

    pub const VALUES: &'static [&'static str; 2] = &[Self::DRAW_CURRENT_ALLOCATIONS, Self::RAW_BALANCE];
}

pub async fn handler_get_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        GetPortfolioButtons::DRAW_CURRENT_ALLOCATIONS => {
            let portfolio = Portfolio::get(q.chat_id().unwrap().0)?;
            let pie_chart = portfolio.draw_pie_current_allocations();

            bot.send_photo(chat_id, pie_chart).await?;

            start_again(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::RAW_BALANCE => {
            let portfolio = Portfolio::get(q.chat_id().unwrap().0)?;
            bot.send_message(chat_id, format!("Ваш портфель: {:#?}", portfolio)).await?;

            start_again(bot, dialogue, chat_id).await?;
        }

        _ => { todo!() }
    }
    Ok(())
}