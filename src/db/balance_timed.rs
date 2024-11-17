use std::str::FromStr;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::db::portfolio::Portfolio;
use crate::enums::category::Category;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::Convert;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BalanceTimed {
    id: Uuid,
    amount: u32,
    category: Option<Category>,
    date: DateTime<Utc>,
}

impl BalanceTimed {
    pub fn load_db(id: String, amount: u32, category: Option<Category>, date: DateTime<Utc>) -> BalanceTimed {
        BalanceTimed {
            id: Uuid::from_str(id.as_str()).unwrap(),
            amount,
            category,
            date,
        }
    }
    pub fn get_id(&self) -> String { self.id.to_string() }
    pub fn get_amount(&self) -> u32 { self.amount }
    pub fn get_amount_bc(&self, portfolio: &Portfolio, current_currency: &Currency) -> u32 {
        portfolio.convert(self.amount, current_currency)
    }
    pub fn get_category(&self) -> Option<Category> { self.category.clone() }
    pub fn get_date(&self) -> DateTime<Utc> { self.date.clone() }

    pub fn new(start_balance: u32) -> Self { BalanceTimed { id: Uuid::new_v4(), amount: start_balance, date: Utc::now(), category: None } }
    pub fn new_date(start_balance: u32, date: DateTime<Utc>) -> Self { BalanceTimed { id: Uuid::new_v4(), amount: start_balance, date, category: None } }
    pub fn new_category(start_balance: u32, category: Option<Category>) -> Self { BalanceTimed { id: Uuid::new_v4(), amount: start_balance, date: Utc::now(), category } }
    pub fn new_date_category(start_balance: u32, date: DateTime<Utc>, category: Option<Category>) -> Self { BalanceTimed { id: Uuid::new_v4(), amount: start_balance, date, category } }
}