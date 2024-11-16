use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::{CallbackQuery, Requester};
use crate::{start_again, HandlerResult, MyDialogue};
use crate::db::db::DataBase;
use crate::db::portfolio::Portfolio;

pub struct Category {}
impl Category {
    pub const APARTMENT_RENT: &'static str = "🏢 Аренда кв";
    pub const CAFES_AND_RESTAURANTS: &'static str = "🍽 Кафе и рестораны";
    pub const CAR: &'static str = "🚗 Машина";
    pub const CLOTH: &'static str = "👕 Одежда";
    pub const EDUCATION: &'static str = "📚 Образование";
    pub const ENTERTAINMENT: &'static str = "🎭 Развлечения";
    pub const FAST_FOOD: &'static str = "🍔 Фастфуд";
    pub const GIFTS: &'static str = "🎁 Подарки";
    pub const PRODUCTS: &'static str = "🛒 Продукты";
    pub const PERSONAL: &'static str = "👤 Личное";
    pub const PETS: &'static str = "🐶 Животные";
    pub const TAXI: &'static str = "🚕 Такси";
    pub const TRANSPORT: &'static str = "🚌 Транспорт";
    pub const TRIPS: &'static str = "✈️ Путешествия";
    pub const HEALTH: &'static str = "💊 Здоровье";
    pub const HOUSE: &'static str = "🏠 Дом";
    pub const OTHER: &'static str = "❓ Другое";

    pub const VALUES: &'static [&'static str; 17] = &[
        Self::APARTMENT_RENT,
        Self::CAFES_AND_RESTAURANTS,
        Self::CAR,
        Self::CLOTH,
        Self::EDUCATION,
        Self::ENTERTAINMENT,
        Self::FAST_FOOD,
        Self::GIFTS,
        Self::OTHER,
        Self::PRODUCTS,
        Self::PERSONAL,
        Self::PETS,
        Self::TAXI,
        Self::TRANSPORT,
        Self::TRIPS,
        Self::HEALTH,
        Self::HOUSE,
    ];
}

pub async fn handler_category_btn(
    bot: Bot,
    dialogue: MyDialogue,
    (balance_name, outcome): (String, u32),
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().unwrap();

    if let Some(ref data) = q.data {
        let category: String = data.as_str().chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect::<String>().trim().to_string();
        let mut portfolio = Portfolio::get(chat_id.0).unwrap_or(Portfolio::empty());

        portfolio.get_account_mut(&*balance_name).unwrap().add_balance_outcome(outcome, category);
        portfolio.save(chat_id)?;

        start_again(bot, dialogue, chat_id).await?;
    } else {
        todo!()
    }
    Ok(())
}