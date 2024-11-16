use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::currency::Currency::*;

#[derive(Clone, Debug, Display, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Currency {
    #[default]
    #[strum(to_string = "RUB")]
    Rub = 0,
    #[strum(to_string = "USD")]
    Usd = 1,
    #[strum(to_string = "EUR")]
    Eur = 2,
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
