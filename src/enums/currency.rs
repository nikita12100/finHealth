use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::currency::Currency::*;

#[derive(Clone, Debug, Display, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Currency {
    #[default]
    #[strum(serialize = "RUB", to_string = "RUB")]
    RUB,
    #[strum(serialize = "USD", to_string = "USD")]
    USD,
    #[strum(serialize = "EUR", to_string = "EUR")]
    EUR,
}

impl Currency {
    pub fn iterator() -> Iter<'static, Currency> {
        static CURRENCY: [Currency; 3] = [RUB, USD, EUR];
        CURRENCY.iter()
    }
}

impl FromStr for Currency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rub" => Ok(RUB),
            "usd" => Ok(USD),
            "eur" => Ok(EUR),
            _ => Err(()),
        }
    }
}
