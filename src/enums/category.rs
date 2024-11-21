use std::slice::Iter;
use strum_macros::{Display, EnumString, IntoStaticStr};
use crate::enums::category::Category::*;

#[derive(Clone, Debug, Default, PartialEq)]
#[derive(Display, EnumString, IntoStaticStr)]
pub enum Category {
    #[default]
    #[strum(serialize = "❓ Другое")]
    Other = 0,
    #[strum(serialize = "🏢 Аренда кв")]
    ApartmentRent = 1,
    #[strum(serialize = "🍽 Кафе и рестораны")]
    CafesAndRestaurants = 2,
    #[strum(serialize = "🚗 Машина")]
    Car = 3,
    #[strum(serialize = "👕 Одежда")]
    Cloth = 4,
    #[strum(serialize = "📚 Образование")]
    Education = 5,
    #[strum(serialize = "🎭 Развлечения")]
    Entertainment = 6,
    #[strum(serialize = "🍔 Фастфуд")]
    FastFood = 7,
    #[strum(serialize = "🎁 Подарки")]
    Gifts = 8,
    #[strum(serialize = "🛒 Продукты")]
    Products = 9,
    #[strum(serialize = "👤 Личное")]
    Personal = 10,
    #[strum(serialize = "🐶 Животные")]
    Pets = 11,
    #[strum(serialize = "🚕 Такси")]
    Taxi = 12,
    #[strum(serialize = "🚌 Транспорт")]
    Transport = 13,
    #[strum(serialize = "✈️ Путешествия")]
    Trips = 14,
    #[strum(serialize = "💊 Здоровье")]
    Health = 15,
    #[strum(serialize = "🏠 Дом")]
    House = 16,
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

impl TryFrom<i32> for Category {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == -1 => Err(()),
            x if x == Other as i32 => Ok(Other),
            x if x == ApartmentRent as i32 => Ok(ApartmentRent),
            x if x == CafesAndRestaurants as i32 => Ok(CafesAndRestaurants),
            x if x == Car as i32 => Ok(Car),
            x if x == Cloth as i32 => Ok(Cloth),
            x if x == Education as i32 => Ok(Education),
            x if x == Entertainment as i32 => Ok(Entertainment),
            x if x == FastFood as i32 => Ok(FastFood),
            x if x == Gifts as i32 => Ok(Gifts),
            x if x == Products as i32 => Ok(Products),
            x if x == Personal as i32 => Ok(Personal),
            x if x == Pets as i32 => Ok(Pets),
            x if x == Taxi as i32 => Ok(Taxi),
            x if x == Transport as i32 => Ok(Transport),
            x if x == Trips as i32 => Ok(Trips),
            x if x == Health as i32 => Ok(Health),
            x if x == House as i32 => Ok(House),
            _ => Err(()),
        }
    }
}
