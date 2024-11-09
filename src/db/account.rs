use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use crate::utils::currency::Currency;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BalanceTimed {
    amount: u32,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
    category: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Account {
    name: String,
    currency: Currency,
    balance: Vec<BalanceTimed>,
}

impl Account {
    pub fn new(name: String, start_balance: u32, currency: Currency) -> Self {
        Account { name, currency, balance: vec![BalanceTimed { amount: start_balance, date: Utc::now(), category: None }] }
    }

    pub fn set_balance_amount(&mut self, new_amount: u32) {
        self.balance.push(BalanceTimed { amount: new_amount, date: Utc::now(), category: None });
    }
    pub fn set_balance_amount_with_category(&mut self, new_amount: u32, category: String) {
        self.balance.push(BalanceTimed { amount: new_amount, date: Utc::now(), category: Some(category) });
    }
    pub fn add_balance_income(&mut self, income: u32) {
        let new_amount = self.balance.last_mut().unwrap().amount + income;
        self.set_balance_amount(new_amount);
    }
    pub fn add_balance_outcome(&mut self, outcome: u32, category: String) {
        let new_amount = self.balance.last_mut().unwrap().amount - outcome;
        self.set_balance_amount_with_category(new_amount, category);
    }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_name_str(&self) -> &str { &self.name }
    pub fn get_last_amount(&self) -> Option<u32> { self.balance.last().map(|x| x.amount) }
}
