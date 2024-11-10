use std::collections::HashMap;
use teloxide::types::InputFile;
use crate::charts::pie_chart::{PieChart, PiePiece};
use crate::db::account::Account;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::ExchangeRate;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Portfolio {
    accounts: Vec<Account>,
    base_currency: Currency,  // add buttons
    exchange_rate: ExchangeRate,  // add buttons
}

impl Portfolio {
    pub fn empty() -> Portfolio {
        Portfolio {
            accounts: Vec::new(),
            base_currency: Currency::default(),
            exchange_rate: ExchangeRate::new(95.0, 100.0, 0.93),
        }
    }

    pub fn get_account_names(&self) -> Vec<String> {
        self.accounts.iter().map(|account| account.get_name().clone()).collect()
    }
    pub fn get_account_names_str(&self) -> Vec<&str> {
        self.accounts.iter().map(|account| account.get_name_str()).collect()
    }
    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    pub fn get_account(&self, name: &str) -> Option<Account> {
        self.accounts.iter().find(|account| account.get_name() == name).cloned()
    }
    pub fn get_account_mut(&mut self, name: &str) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|account| account.get_name() == name)
    }

    fn draw_pie_from_distribution(distribution: HashMap<String, u32>, title: &str) -> InputFile {
        let mut parts: Vec<PiePiece> = Vec::new();
        let mut total_summ = 0;
        for (key, value) in distribution {
            total_summ += value;
            parts.push(PiePiece { size: value as f64, label: key });
        }

        PieChart::create(parts, title, Some(Self::total_sum_spaced(total_summ)))
    }

    pub fn draw_pie_type_allocations(&self) -> InputFile {
        let mut distribution_type: HashMap<String, u32> = HashMap::new();
        for account in self.accounts.iter() {
            distribution_type
                .entry(account.get_type().to_string())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap())
                .or_insert(account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_type, "Срез по всем балансам в типах")
    }

    pub fn draw_pie_location_allocations(&self) -> InputFile {
        let mut distribution_location: HashMap<String, u32> = HashMap::new();
        for account in self.accounts.iter() {
            distribution_location
                .entry(account.get_location().to_string())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap())
                .or_insert(account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_location, "Срез по всем балансам в локациях")
    }

    pub fn draw_pie_currency_allocations(&self) -> InputFile {
        let mut distribution_currency: HashMap<String, u32> = HashMap::new();
        for account in self.accounts.iter() {
            distribution_currency
                .entry(account.get_currency().to_string())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap())
                .or_insert(account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_currency, "Срез по всем балансам в валютах")
    }

    pub fn draw_pie_current_allocations(&self) -> InputFile {
        let mut distribution_amount: HashMap<String, u32> = HashMap::new();
        for account in self.accounts.iter() {
            distribution_amount
                .entry(account.get_name())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap())
                .or_insert(account.get_last_amount_bc(&self.exchange_rate, self.base_currency.clone()).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_amount, "Срез по всем балансам")
    }


    fn total_sum_spaced(total_summ: u32) -> String {
        let mut total_sum_str: Vec<char> = Vec::new();
        for (i, char) in total_summ.to_string().chars().rev().enumerate() {
            if i % 3 == 0 {
                total_sum_str.push(' ');
                total_sum_str.push(char);
            } else {
                total_sum_str.push(char);
            }
        }
        total_sum_str.reverse();
        total_sum_str.iter().collect()
    }
}
