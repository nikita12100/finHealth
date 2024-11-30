use crate::db::account::Account;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::ExchangeRate;

#[derive(Clone, Debug, Default)]
pub struct Portfolio {
    accounts: Vec<Account>,
    base_currency: Currency,
    exchange_rate: ExchangeRate,
}

impl Portfolio {
    pub fn empty() -> Portfolio {
        Portfolio {
            accounts: Vec::new(),
            base_currency: Currency::default(),
            exchange_rate: ExchangeRate::new(95.0, 100.0, 0.93),
        }
    }
    pub fn new(
        accounts: Vec<Account>,
        base_currency: Currency,
        exchange_rate: ExchangeRate,
    ) -> Portfolio {
        Portfolio {
            accounts,
            base_currency,
            exchange_rate,
        }
    }

    pub fn get_all_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    pub fn get_exchange_rate(&self) -> &ExchangeRate {
        &self.exchange_rate
    }
    pub fn get_base_currency(&self) -> &Currency {
        &self.base_currency
    }
    pub fn set_base_currency(&mut self, c: Currency) {
        self.base_currency = c
    }
    pub fn get_account_names(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.get_name().clone())
            .collect()
    }
    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    pub fn get_account(&self, name: &str) -> Option<Account> {
        self.accounts
            .iter()
            .find(|account| account.get_name() == name)
            .cloned()
    }
    pub fn get_account_mut(&mut self, name: &str) -> Option<&mut Account> {
        self.accounts
            .iter_mut()
            .find(|account| account.get_name() == name)
    }
}
