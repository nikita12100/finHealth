use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::ExchangeRate;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BalanceTimed {
    amount: u32,
    category: Option<String>,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Account {
    name: String,
    currency: Currency, // add buttons
    // risk_level: u32,
    asset_location: AssetLocation, // add buttons
    asset_type: AssetType, // add buttons
    balance: Vec<BalanceTimed>,
}

impl Account {
    pub fn new(name: String, start_balance: u32, currency: Currency, asset_location: AssetLocation, asset_type: AssetType) -> Self {
        Account {
            name,
            currency,
            asset_location,
            asset_type,
            balance: vec![BalanceTimed { amount: start_balance, date: Utc::now(), category: None }],
        }
    }

    pub fn set_balance_amount(&mut self, new_amount: u32, category: Option<String>) {
        self.balance.push(BalanceTimed { amount: new_amount, date: Utc::now(), category });
    }
    fn set_balance_amount_date(&mut self, new_amount: u32, category: Option<String>, date: DateTime<Utc>) {
        self.balance.push(BalanceTimed { amount: new_amount, date, category });
    }
    pub fn add_balance_income(&mut self, income: u32) {
        let new_amount = self.balance.last_mut().unwrap().amount + income;
        self.set_balance_amount(new_amount, None);
    }
    pub fn add_balance_outcome(&mut self, outcome: u32, category: String) {
        let new_amount = self.balance.last_mut().unwrap().amount - outcome;
        self.set_balance_amount(new_amount, Some(category));
    }
    pub fn add_balance_outcome_date(&mut self, outcome: u32, category: String, date: DateTime<Utc>) {
        let new_amount = self.balance.last_mut().unwrap().amount - outcome;
        self.set_balance_amount_date(new_amount, Some(category), date);
    }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_name_str(&self) -> &str { &self.name }
    pub fn get_currency(&self) -> Currency { self.currency.clone() }
    pub fn get_location(&self) -> AssetLocation { self.asset_location.clone() }
    pub fn get_type(&self) -> AssetType { self.asset_type.clone() }
    pub fn get_last_amount(&self) -> Option<u32> { self.balance.last().map(|x| x.amount) }
    pub fn get_last_amount_bc(&self, exchange: &ExchangeRate, base_currency: Currency) -> Option<u32> {
        if let Some(balance) = self.balance.last().map(|x| x.amount) {
            Some(exchange.convert(balance as f32, self.currency.clone(), base_currency) as u32)
        } else {
            None
        }
    }
}
