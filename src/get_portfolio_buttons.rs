use std::io::Read;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::types::InputFile;
use teloxide::types::PassportElementErrorUnspecifiedType::File;
use crate::{make_keyboard, start_again, HandlerResult, MyDialogue, State};
use crate::dao::Portfolio;
use crate::db::DataBase;
use crate::pie_chart::PieChart;

pub struct GetPortfolioButtons {}

impl GetPortfolioButtons {
    pub const DRAW_BALANCE: &'static str = "Показать отчет";
    pub const RAW_BALANCE: &'static str = "[DEV] Показать баланс";

    pub const VALUES: &'static [&'static str; 2] = &[Self::DRAW_BALANCE, Self::RAW_BALANCE];
}

pub async fn handler_get_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    match q.data.clone().unwrap().as_str() {
        GetPortfolioButtons::DRAW_BALANCE => {
            let portfolio = Portfolio::get(q.chat_id().unwrap().0)?;

            let pie_chart = PieChart::create_file();

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