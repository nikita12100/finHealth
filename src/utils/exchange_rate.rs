use crate::db::portfolio::Portfolio;
use crate::enums::currency::Currency;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ExchangeRate {
    usd_rub: f32,
    usd_eur: f32,

    rub_usd: f32,
    rub_eur: f32,

    eur_rub: f32,
    eur_usd: f32,
}
impl ExchangeRate {
    pub fn new(usd_rub: f32, eur_rub: f32, usd_eur: f32) -> ExchangeRate {
        ExchangeRate {
            usd_rub,
            usd_eur,
            rub_usd: 1.0 / usd_rub,
            rub_eur: 1.0 / eur_rub,
            eur_rub,
            eur_usd: 1.0 / usd_eur,
        }
    }
    pub fn convert(&self, amount: f32, from: &Currency, to: &Currency) -> f32 {
        if from.eq(to) { amount } else {
            match from {
                Currency::RUB => {
                    match to {
                        Currency::RUB => amount,
                        Currency::USD => amount * self.rub_usd,
                        Currency::EUR => amount * self.rub_eur,
                    }
                }
                Currency::USD => {
                    match to {
                        Currency::RUB => amount * self.usd_rub,
                        Currency::USD => amount,
                        Currency::EUR => amount * self.usd_eur,
                    }
                }
                Currency::EUR => {
                    match to {
                        Currency::RUB => amount * self.eur_usd,
                        Currency::USD => amount * self.eur_usd,
                        Currency::EUR => amount,
                    }
                }
            }
        }
    }
}

pub trait Convert {
    fn convert(&self, amount: u32, from: &Currency) -> u32;
}

impl Convert for Portfolio {
    fn convert(&self, amount: u32, from: &Currency) -> u32 {
        self.get_exchange_rate().convert(amount as f32, from, self.get_base_currency()) as u32
    }
}
