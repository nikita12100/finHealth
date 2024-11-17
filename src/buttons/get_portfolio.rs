use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{goto_start, invalid_input_for_callback, HandlerResult, MyDialogue};
use crate::charts::draw_line::DrawLine;
use crate::charts::draw_pie::DrawPie;
use crate::db::portfolio::Portfolio;
use crate::db::db::DataBase;

pub struct GetPortfolioButtons {}

impl GetPortfolioButtons {
    pub const DRAW_NAME_ALLOCATIONS: &'static str = "Срез по имени баланса";
    pub const DRAW_CURRENCY_ALLOCATIONS: &'static str = "Срез по валютам актива";
    pub const DRAW_LOCATION_ALLOCATIONS: &'static str = "Срез по хранению актива";
    pub const DRAW_TYPE_ALLOCATIONS: &'static str = "Срез по типу актива";
    pub const DRAW_WEEK_SPENDS: &'static str = "Срез дейли трат за неделю";
    pub const DRAW_MONTH_SPENDS: &'static str = "Срез дейли трат за месяц";
    pub const DRAW_LINE_ALL_HIST: &'static str = "Историчность по всем счетам";
    // pub const DRAW_CURRENT_ALLOCATIONS: &'static str = "Показать траты за все время по балансу";
    pub const RAW_BALANCE: &'static str = "[DEV] Показать сырой баланс";

    pub const VALUES: &'static [&'static str; 8] = &[
        Self::DRAW_NAME_ALLOCATIONS,
        Self::DRAW_CURRENCY_ALLOCATIONS,
        Self::DRAW_LOCATION_ALLOCATIONS,
        Self::DRAW_TYPE_ALLOCATIONS,
        Self::DRAW_WEEK_SPENDS,
        Self::DRAW_MONTH_SPENDS,
        Self::DRAW_LINE_ALL_HIST,
        Self::RAW_BALANCE
    ];
}

pub async fn handler_get_portfolio_btn(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();
    let portfolio = Portfolio::get(q.chat_id().unwrap().0).unwrap();

    match q.data.clone().unwrap().as_str() {
        GetPortfolioButtons::DRAW_NAME_ALLOCATIONS => {
            let pie_chart = portfolio.draw_pie_name_allocations();

            bot.send_photo(chat_id, pie_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::DRAW_CURRENCY_ALLOCATIONS => {
            let pie_chart = portfolio.draw_pie_currency_allocations();

            bot.send_photo(chat_id, pie_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::DRAW_LOCATION_ALLOCATIONS => {
            let pie_chart = portfolio.draw_pie_location_allocations();

            bot.send_photo(chat_id, pie_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::DRAW_TYPE_ALLOCATIONS => {
            let pie_chart = portfolio.draw_pie_type_allocations();

            bot.send_photo(chat_id, pie_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::DRAW_WEEK_SPENDS => {
            let pie_chart = portfolio.draw_pie_spends("daily".to_string(), 7);

            bot.send_photo(chat_id, pie_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::DRAW_MONTH_SPENDS => {
            let pie_chart = portfolio.draw_pie_spends("daily".to_string(), 30);

            bot.send_photo(chat_id, pie_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::DRAW_LINE_ALL_HIST => {
            let line_chart = portfolio.draw_line_test();

            bot.send_photo(chat_id, line_chart).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        GetPortfolioButtons::RAW_BALANCE => {
            bot.send_message(chat_id, format!("Ваш портфель: {:#?}", portfolio)).await?;
            goto_start(bot, dialogue, chat_id).await?;
        }
        _ => {
            invalid_input_for_callback(bot, dialogue, q, format!("Необходимо выбрать одну из кнопок {:?}", GetPortfolioButtons::VALUES.to_vec())).await?;
        }
    }
    Ok(())
}