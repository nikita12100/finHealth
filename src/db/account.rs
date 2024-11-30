use crate::db::balance_timed::BalanceTimed;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::category::Category;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::Convert;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Account {
    id: Uuid,
    name: String,
    currency: Currency,
    // risk_level: u32,
    asset_location: AssetLocation,
    asset_type: AssetType,
    balance: Vec<BalanceTimed>,
}

impl Account {
    pub fn new_date(
        name: String,
        start_balance: u32,
        currency: Currency,
        asset_location: AssetLocation,
        asset_type: AssetType,
        date: DateTime<Utc>,
    ) -> Self {
        Account {
            id: Uuid::new_v4(),
            name,
            currency,
            asset_location,
            asset_type,
            balance: vec![BalanceTimed::new_date(start_balance, date)],
        }
    }
    pub fn load_db(
        id: String,
        name: String,
        currency: Currency,
        asset_location: AssetLocation,
        asset_type: AssetType,
        balance: Vec<BalanceTimed>,
    ) -> Self {
        Account {
            id: Uuid::from_str(id.as_str()).unwrap(),
            name,
            currency,
            asset_location,
            asset_type,
            balance,
        }
    }
    pub fn append_balance_db(&mut self, balances: Vec<BalanceTimed>) {
        self.balance = balances;
    }
    pub fn new(
        name: String,
        start_balance: u32,
        currency: Currency,
        asset_location: AssetLocation,
        asset_type: AssetType,
    ) -> Self {
        Account {
            id: Uuid::new_v4(),
            name,
            currency,
            asset_location,
            asset_type,
            balance: vec![BalanceTimed::new(start_balance)],
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
    fn set_balance_amount(&mut self, new_amount: u32, category: Option<Category>) {
        self.balance
            .push(BalanceTimed::new_category(new_amount, category));
    }
    fn set_balance_amount_date(
        &mut self,
        new_amount: u32,
        category: Option<Category>,
        date: DateTime<Utc>,
    ) {
        self.balance
            .push(BalanceTimed::new_date_category(new_amount, date, category));
    }
    pub fn add_balance_income(&mut self, income: u32) {
        let new_amount = self.balance.last().unwrap().get_amount() + income;
        self.set_balance_amount(new_amount, None);
    }
    pub fn add_balance_income_date(&mut self, income: u32, date: DateTime<Utc>) {
        let new_amount = self.balance.last().unwrap().get_amount() + income;
        self.set_balance_amount_date(new_amount, None, date);
    }
    pub fn add_balance_outcome(&mut self, outcome: u32, category: Category) {
        let new_amount = self.balance.last().unwrap().get_amount() - outcome; // Todo вычитание меньше 0
        self.set_balance_amount(new_amount, Some(category));
    }
    pub fn add_balance_outcome_date(
        &mut self,
        outcome: u32,
        category: Category,
        date: DateTime<Utc>,
    ) {
        let new_amount = self.balance.last().unwrap().get_amount() - outcome;
        self.set_balance_amount_date(new_amount, Some(category), date);
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_currency(&mut self, c: Currency) {
        self.currency = c
    }

    pub fn get_location(&self) -> AssetLocation {
        self.asset_location.clone()
    }
    pub fn set_location(&mut self, l: AssetLocation) {
        self.asset_location = l
    }

    pub fn get_type(&self) -> AssetType {
        self.asset_type.clone()
    }
    pub fn set_type(&mut self, t: AssetType) {
        self.asset_type = t
    }

    pub fn get_last_amount(&self) -> Option<u32> {
        self.balance.last().map(|x| x.get_amount())
    }

    pub fn get_last_amount_bc(&self, portfolio: &Portfolio) -> Option<u32> {
        if let Some(balance_amount) = self.balance.last().map(|x| x.get_amount()) {
            Some(portfolio.convert(balance_amount, &self.currency))
        } else {
            None
        }
    }
    pub fn get_balances(&self) -> Vec<BalanceTimed> {
        self.balance.clone()
    }
}
