use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::currency::Currency::*;

#[derive(Clone, Debug, Display, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Currency {
    #[default]
    #[strum(serialize = "RUB", to_string = "RUB")]
    Rub,
    #[strum(serialize = "USD", to_string = "USD")]
    Usd,
    #[strum(serialize = "EUR", to_string = "EUR")]
    Eur,
}

impl Currency {
    pub fn iterator() -> Iter<'static, Currency> {
        static VALUES: [Currency; 3] = [Rub, Usd, Eur];
        VALUES.iter()
    }
}

impl FromStr for Currency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rub" => Ok(Rub),
            "usd" => Ok(Usd),
            "eur" => Ok(Eur),
            _ => Err(()),
        }
    }
}
