use crate::charts::draw_line::DrawLine;
use crate::charts::draw_pie::DrawPie;
use crate::db::database::db_portfolio::DataBasePortfolio;
use crate::db::portfolio::Portfolio;
use crate::enums::state::State;
use crate::utils::common::make_keyboard_string;
use crate::{
    get_or_create_portfolio, goto_start, invalid_input_for_callback, HandlerResult, MyDialogue,
};
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::Bot;

pub struct GetPortfolioButtons {}

impl GetPortfolioButtons {
    pub const DRAW_NAME_ALLOCATIONS: &'static str = "🍕 Срез по имени баланса";
    pub const DRAW_CURRENCY_ALLOCATIONS: &'static str = "🍕 Срез по валютам актива";
    pub const DRAW_LOCATION_ALLOCATIONS: &'static str = "🍕 Срез по хранению актива";
    pub const DRAW_TYPE_ALLOCATIONS: &'static str = "🍕 Срез по типу актива";
    pub const DRAW_WEEK_SPENDS: &'static str = "🍕 Срез дейли трат за неделю";
    pub const DRAW_MONTH_SPENDS: &'static str = "🍕 Срез дейли трат за месяц";
    pub const DRAW_LINE_ALL_HIST: &'static str = "📊 Историчность по всем счетам";
    // pub const DRAW_CURRENT_ALLOCATIONS: &'static str = "Показать траты за все время по балансу";
    pub const RAW_BALANCE: &'static str = "⚙️ [DEV] Показать сырой баланс";

    pub const VALUES: &'static [&'static str; 8] = &[
        Self::DRAW_NAME_ALLOCATIONS,
        Self::DRAW_CURRENCY_ALLOCATIONS,
        Self::DRAW_LOCATION_ALLOCATIONS,
        Self::DRAW_TYPE_ALLOCATIONS,
        Self::DRAW_WEEK_SPENDS,
        Self::DRAW_MONTH_SPENDS,
        Self::DRAW_LINE_ALL_HIST,
        Self::RAW_BALANCE,
    ];
}

pub async fn handler_get_portfolio_btn(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(portfolio) = Portfolio::get(q.chat_id().unwrap().0) {
        match q.data.clone().unwrap().as_str() {
            GetPortfolioButtons::DRAW_NAME_ALLOCATIONS => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Диаграмма среза счетов по имени",
                )
                .await?;

                let pie_chart = portfolio.draw_pie_name_allocations();

                bot.send_photo(chat_id, pie_chart).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            }
            GetPortfolioButtons::DRAW_CURRENCY_ALLOCATIONS => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Диаграмма среза счетов по валюте",
                )
                .await?;

                let pie_chart = portfolio.draw_pie_currency_allocations();

                bot.send_photo(chat_id, pie_chart).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            }
            GetPortfolioButtons::DRAW_LOCATION_ALLOCATIONS => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Диаграмма среза счетов по локации",
                )
                .await?;

                let pie_chart = portfolio.draw_pie_location_allocations();

                bot.send_photo(chat_id, pie_chart).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            }
            GetPortfolioButtons::DRAW_TYPE_ALLOCATIONS => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Диаграмма среза счетов по типу",
                )
                .await?;

                let pie_chart = portfolio.draw_pie_type_allocations();

                bot.send_photo(chat_id, pie_chart).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            }
            GetPortfolioButtons::DRAW_WEEK_SPENDS => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Для какого портфеля показать недельные траты?",
                )
                .await?;

                let accounts_name = portfolio.get_account_names();
                dialogue
                    .update(State::ListenBalanceNameSpendsCallback(7))
                    .await?;
                bot.send_message(chat_id, "Выберите какой баланс вы хотите изменить:")
                    .reply_markup(make_keyboard_string(1, accounts_name))
                    .await?;
            }
            GetPortfolioButtons::DRAW_MONTH_SPENDS => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "Для какого портфеля показать месячные траты?",
                )
                .await?;

                let accounts_name = portfolio.get_account_names();
                dialogue
                    .update(State::ListenBalanceNameSpendsCallback(30))
                    .await?;
                bot.send_message(chat_id, "Выберите какой баланс вы хотите изменить:")
                    .reply_markup(make_keyboard_string(1, accounts_name))
                    .await?;
            }
            GetPortfolioButtons::DRAW_LINE_ALL_HIST => {
                bot.edit_message_text(
                    chat_id,
                    q.message.clone().unwrap().id(),
                    "График историчности",
                )
                .await?;

                let line_chart = portfolio.draw_line_test();

                bot.send_photo(chat_id, line_chart).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            }
            GetPortfolioButtons::RAW_BALANCE => {
                bot.send_message(chat_id, format!("{:#?}", portfolio.get_base_currency()))
                    .await?;
                bot.send_message(chat_id, format!("{:#?}", portfolio.get_exchange_rate()))
                    .await?;
                let accounts: Vec<String> = portfolio.get_all_accounts().iter().map(|a| a.print()).collect::<Vec<_>>();
                bot.send_message(chat_id, format!("{:#?}", accounts))
                    .await?;

                goto_start(bot, dialogue, chat_id, None).await?;
            }
            _ => {
                invalid_input_for_callback(
                    bot,
                    dialogue,
                    q,
                    format!(
                        "Необходимо выбрать одну из кнопок {:?}",
                        GetPortfolioButtons::VALUES.to_vec()
                    ),
                )
                .await?;
            }
        }
    } else {
        let error = "У вас нет счетов, необходимо их добавить";
        goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
    }
    Ok(())
}

pub async fn handler_get_spends_btn(
    bot: Bot,
    dialogue: MyDialogue,
    num_days: u32,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    let portfolio = get_or_create_portfolio(chat_id);
    let balance_name = q.data.clone().unwrap().to_string();
    let accounts_name = portfolio.get_account_names();
    assert!(accounts_name.contains(&balance_name));

    let balances = portfolio
        .get_account(&*balance_name)
        .unwrap()
        .get_balances();
    let spends: Vec<_> = balances
        .iter()
        .filter(|balance| balance.get_category().is_some())
        .collect();
    if spends.len() < 1 {
        bot.edit_message_text(
            chat_id,
            q.message.clone().unwrap().id(),
            "У вас еще нет трат, необходимо их добавить",
        )
        .await?;
        goto_start(bot, dialogue, chat_id, None).await?;
    } else {
        bot.edit_message_text(
            chat_id,
            q.message.clone().unwrap().id(),
            format!(
                "Статистика трат по счету {} за {} дней",
                balance_name, num_days
            ),
        )
        .await?;

        let pie_chart = portfolio.draw_pie_spends(balance_name, num_days);

        bot.send_photo(chat_id, pie_chart).await?;
        goto_start(bot, dialogue, chat_id, None).await?;
    }

    Ok(())
}
