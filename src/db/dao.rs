use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use crate::utils::currency::Currency;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BalanceTimed {
    amount: u32,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
    comment: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Account {
    name: String,
    currency: Currency,
    balance: Vec<BalanceTimed>,
}

impl Account {
    pub fn new(name: String, start_balance: u32, currency: Currency) -> Self {
        Account { name, currency, balance: vec![BalanceTimed { amount: start_balance, date: Utc::now(), comment: None }] }
    }

    pub fn add_balance_record(&mut self, balance: u32) {
        self.balance.push(BalanceTimed { amount: balance, date: Utc::now(), comment: None });
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Portfolio {
    accounts: Vec<Account>,
}

/// get day/week/month stats, get sum, get last data, convert to rub/usd
/// Цель 0 — пенсия
/// Репорт о прошедшем месяце
/// Репорт по отложенным средствам
/// сортировака по расходам за неделю\месяц
impl Portfolio {
    pub fn empty() -> Portfolio {
        Portfolio { accounts: Vec::new() }
    }

    pub fn get_account_names(&self) -> Vec<String> { self.accounts.iter().map(|account| account.name.clone()).collect() }
    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn add_account_record(&mut self, name: &str, _amount: u32) {
        for account in self.accounts.iter_mut() {
            if account.name == name {
                account.add_balance_record(_amount);
            }
        }
    }
}
