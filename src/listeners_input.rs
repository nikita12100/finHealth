use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use crate::{goto_start, init_portfolio, HandlerResult, MyDialogue, State};
use crate::buttons::account::set_category::ButtonCategory;
use crate::db::account::Account;
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::currency::Currency;
use crate::utils::common::make_keyboard_string;

pub async fn listen_new_account_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(name) => {
            bot.send_message(msg.chat.id, format!("Укажите баланс для счета  {:#?}:", name)).await?;
            dialogue.update(State::GotNewAccountName(name.to_string())).await?;
        }
        None => {
            let text = "Неправильное имя счета";
            goto_start(bot, dialogue, msg.chat.id, Some(text.to_string())).await?;
        }
    }
    Ok(())
}


pub async fn listen_new_account_amount(
    bot: Bot,
    dialogue: MyDialogue,
    account_name: String,
    msg: Message,
) -> HandlerResult {
    let chat_id = msg.chat.id;

    match msg.text().unwrap().parse::<u32>() {
        Ok(amount) => {
            let account = Account::new(account_name.clone(), amount, Currency::Rub, AssetLocation::Other, AssetType::default());
            if let Some(mut portfolio) = Portfolio::get(chat_id.0) {
                portfolio.add_account(account);
                portfolio.save(chat_id)?;

                bot.send_message(chat_id, format!("Счет \"{}\" успешно добавлен", &account_name)).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            } else {
                log::error!("Portfolio not found for {}", chat_id);
                init_portfolio(chat_id)?;
                let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
                goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
            }
        }
        Err(_) => {
            let text = "Неправильное значение баланса";
            goto_start(bot, dialogue, chat_id, Some(text.to_string())).await?;
        }
    }
    Ok(())
}

pub async fn listen_account_income_amount(
    bot: Bot,
    dialogue: MyDialogue,
    account_name: String,
    msg: Message,
) -> HandlerResult {
    let chat_id = msg.chat.id;

    match msg.text().unwrap().parse::<u32>() {
        Ok(income) => {
            if let Some(mut portfolio) = Portfolio::get(chat_id.0) {
                portfolio.get_account_mut(&*account_name).unwrap().add_balance_income(income);
                portfolio.save(chat_id)?;
                let account = portfolio.get_account(&*account_name).unwrap();
                let last_amount = account.get_last_amount().unwrap();
                bot.send_message(chat_id, format!("Счет \"{}\" успешно обновлен, текущий баланс {}", account_name, last_amount)).await?;
                goto_start(bot, dialogue, chat_id, None).await?;
            } else {
                log::error!("Portfolio not found for {}", chat_id);
                init_portfolio(chat_id)?;
                let error = "Простите, произошла ошибка :(\nCode 1\nПовторите операцию";
                goto_start(bot, dialogue, chat_id, Some(error.to_string())).await?;
            }
        }
        Err(_) => {
            let text = "Неправильное значение баланса";
            goto_start(bot, dialogue, chat_id, Some(text.to_string())).await?;
        }
    }
    Ok(())
}

pub async fn listen_account_outcome_amount(
    bot: Bot,
    dialogue: MyDialogue,
    account_name: String,
    msg: Message,
) -> HandlerResult {
    let chat_id = msg.chat.id;

    match msg.text().unwrap().parse::<u32>() {
        Ok(outcome) => {
            dialogue.update(State::ListenCategoryCallback { account_name, outcome }).await?;

            bot.send_message(chat_id, "Выберите категорию трат:").reply_markup(make_keyboard_string(3, ButtonCategory::get_categories())).await?;
        }
        Err(_) => {
            let text = "Неправильное значение баланса";
            goto_start(bot, dialogue, chat_id, Some(text.to_string())).await?;
        }
    }
    Ok(())
}
