use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use crate::db::portfolio::Portfolio;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::Convert;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct BalanceTimed {
    amount: u32,
    category: Option<String>,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

impl BalanceTimed {
    pub fn get_amount(&self) -> u32 { self.amount }
    pub fn get_amount_bc(&self, portfolio: &Portfolio, current_currency: &Currency) -> u32 {
        portfolio.convert(self.amount, current_currency)
    }
    pub fn get_category(&self) -> Option<String> { self.category.clone() }
    pub fn get_date(&self) -> DateTime<Utc> { self.date.clone() }

    pub fn new(start_balance: u32) -> Self { BalanceTimed { amount: start_balance, date: Utc::now(), category: None } }
    pub fn new_date(start_balance: u32, date: DateTime<Utc>) -> Self { BalanceTimed { amount: start_balance, date, category: None } }
    pub fn new_category(start_balance: u32, category: Option<String>) -> Self { BalanceTimed { amount: start_balance, date: Utc::now(), category } }
    pub fn new_date_category(start_balance: u32, date: DateTime<Utc>, category: Option<String>) -> Self { BalanceTimed { amount: start_balance, date, category } }
}