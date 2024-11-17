use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use crate::{goto_start, HandlerResult, MyDialogue, State};
use crate::buttons::account::set_category::ButtonCategory;
use crate::db::account::Account;
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::currency::Currency;
use crate::utils::common::make_keyboard_string;

pub async fn listen_new_balance_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(name) => {
            bot.send_message(msg.chat.id, format!("name will be {:#?}, write please amount:", name)).await?;
            dialogue.update(State::GotNewBalanceName(name.to_string())).await?;
            Ok(())
        }
        None => { panic!("Error parsing answer") }
    }
}


pub async fn listen_new_balance_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(amount) => {
            let balance = Account::new(balance_name, amount, Currency::Rub, AssetLocation::Other, AssetType::default());
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.add_account(balance);
            portfolio.save(msg.chat.id)?;

            bot.send_message(msg.chat.id, format!("portfolio saved {:#?}", portfolio)).await?;
            goto_start(bot, dialogue, msg.chat.id).await?;

            Ok(())
        }
        Err(_) => { panic!("Error parsing answer") }
    }
}
pub async fn listen_balance_new_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(new_balance) => {
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.get_account_mut(&*balance_name).unwrap().set_balance_amount(new_balance, None);
            portfolio.save(msg.chat.id)?;
            bot.send_message(msg.chat.id, format!("portfolio updated {:#?}", portfolio)).await?;
            goto_start(bot, dialogue, msg.chat.id).await?;
        }
        Err(_) => { panic!("Error parsing answer") }
    }
    Ok(())
}

pub async fn listen_balance_income_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(income) => {
            let mut portfolio = Portfolio::get(msg.chat.id.0).unwrap_or(Portfolio::empty());

            portfolio.get_account_mut(&*balance_name).unwrap().add_balance_income(income);
            portfolio.save(msg.chat.id)?;
            bot.send_message(msg.chat.id, format!("portfolio updated {:#?}", portfolio)).await?;
            goto_start(bot, dialogue, msg.chat.id).await?;
        }
        Err(_) => { panic!("Error parsing answer") }
    }
    Ok(())
}

pub async fn listen_balance_outcome_amount(
    bot: Bot,
    dialogue: MyDialogue,
    balance_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().unwrap().parse::<u32>() {
        Ok(outcome) => {
            dialogue.update(State::ListenCategoryCallback { balance_name, outcome }).await?;

            bot.send_message(msg.chat.id, "Выберите категорию трат:").reply_markup(make_keyboard_string(3, ButtonCategory::get_categories())).await?;
        }
        Err(_) => { panic!("Error parsing answer") }
    }
    Ok(())
}
