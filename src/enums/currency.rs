use crate::enums::currency::Currency::*;
use std::slice::Iter;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Clone, Debug, Default, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum Currency {
    #[default]
    #[strum(serialize = "RUB")]
    Rub = 0,
    #[strum(serialize = "USD")]
    Usd = 1,
    #[strum(serialize = "EUR")]
    Eur = 2,
}

impl Currency {
    pub fn iterator() -> Iter<'static, Currency> {
        static VALUES: [Currency; 3] = [Rub, Usd, Eur];
        VALUES.iter()
    }
}

impl TryFrom<i32> for Currency {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Rub as i32 => Ok(Rub),
            x if x == Usd as i32 => Ok(Usd),
            x if x == Eur as i32 => Ok(Eur),
            _ => Err(()),
        }
    }
}
