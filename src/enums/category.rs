use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::category::Category::*;

#[derive(Clone, Debug, Display, Default, PartialEq)]
pub enum Category {
    #[default]
    #[strum(to_string = "❓ Другое")]
    Other = 0,
    #[strum(to_string = "🏢 Аренда кв")]
    ApartmentRent = 1,
    #[strum(to_string = "🍽 Кафе и рестораны")]
    CafesAndRestaurants = 2,
    #[strum(to_string = "🚗 Машина")]
    Car = 3,
    #[strum(to_string = "👕 Одежда")]
    Cloth = 4,
    #[strum(to_string = "📚 Образование")]
    Education = 5,
    #[strum(to_string = "🎭 Развлечения")]
    Entertainment = 6,
    #[strum(to_string = "🍔 Фастфуд")]
    FastFood = 7,
    #[strum(to_string = "🎁 Подарки")]
    Gifts = 8,
    #[strum(to_string = "🛒 Продукты")]
    Products = 9,
    #[strum(to_string = "👤 Личное")]
    Personal = 10,
    #[strum(to_string = "🐶 Животные")]
    Pets = 11,
    #[strum(to_string = "🚕 Такси")]
    Taxi = 12,
    #[strum(to_string = "🚌 Транспорт")]
    Transport = 13,
    #[strum(to_string = "✈️ Путешествия")]
    Trips = 14,
    #[strum(to_string = "💊 Здоровье")]
    Health = 15,
    #[strum(to_string = "🏠 Дом")]
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

impl FromStr for Category {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ApartmentRent" => Ok(ApartmentRent),
            "CafesAndRestaurants" => Ok(CafesAndRestaurants),
            "Car" => Ok(Car),
            "Cloth" => Ok(Cloth),
            "Education" => Ok(Education),
            "Entertainment" => Ok(Entertainment),
            "FastFood" => Ok(FastFood),
            "Gifts" => Ok(Gifts),
            "Products" => Ok(Products),
            "Personal" => Ok(Personal),
            "Pets" => Ok(Pets),
            "Taxi" => Ok(Taxi),
            "Transport" => Ok(Transport),
            "Trips" => Ok(Trips),
            "Health" => Ok(Health),
            "House" => Ok(House),
            "Other" => Ok(Other),
            _ => Err(()),
        }
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
