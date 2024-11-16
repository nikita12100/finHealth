use std::slice::Iter;
use strum_macros::Display;
use crate::enums::category::Category::*;

#[derive(Clone, Debug, Display, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Category { // todo serde in integer
    #[strum(serialize = "ApartmentRent", to_string = "🏢 Аренда кв")]
    ApartmentRent,
    #[strum(serialize = "CafesAndRestaurants", to_string = "🍽 Кафе и рестораны")]
    CafesAndRestaurants,
    #[strum(serialize = "Car", to_string = "🚗 Машина")]
    Car,
    #[strum(serialize = "Cloth", to_string = "👕 Одежда")]
    Cloth,
    #[strum(serialize = "Education", to_string = "📚 Образование")]
    Education,
    #[strum(serialize = "Entertainment", to_string = "🎭 Развлечения")]
    Entertainment,
    #[strum(serialize = "FastFood", to_string = "🍔 Фастфуд")]
    FastFood,
    #[strum(serialize = "Gifts", to_string = "🎁 Подарки")]
    Gifts,
    #[strum(serialize = "Products", to_string = "🛒 Продукты")]
    Products,
    #[strum(serialize = "Personal", to_string = "👤 Личное")]
    Personal,
    #[strum(serialize = "Pets", to_string = "🐶 Животные")]
    Pets,
    #[strum(serialize = "Taxi", to_string = "🚕 Такси")]
    Taxi,
    #[strum(serialize = "Transport", to_string = "🚌 Транспорт")]
    Transport,
    #[strum(serialize = "Trips", to_string = "✈️ Путешествия")]
    Trips,
    #[strum(serialize = "Health", to_string = "💊 Здоровье")]
    Health,
    #[strum(serialize = "House", to_string = "🏠 Дом")]
    House,
    #[default]
    #[strum(serialize = "Other", to_string = "❓ Другое")]
    Other,
}

impl Category {
    pub fn iterator() -> Iter<'static, Category> {
        static VALUES: [Category; 17] = [
            ApartmentRent,
            CafesAndRestaurants,
            Car,
            Cloth,
            Education,
            Entertainment,
            FastFood,
            Gifts,
            Other,
            Products,
            Personal,
            Pets,
            Taxi,
            Transport,
            Trips,
            Health,
            House,
        ];
        VALUES.iter()
    }
}
