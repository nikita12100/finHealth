use std::collections::HashMap;
use teloxide::types::InputFile;
use crate::charts::pie_chart::{PieChart, PiePiece};
use crate::db::account::Account;
use crate::utils::currency::Currency;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Portfolio {
    accounts: Vec<Account>,
}

impl Portfolio {
    pub fn empty() -> Portfolio {
        Portfolio { accounts: Vec::new() }
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

    pub fn draw_pie_currency_allocations(&self) -> InputFile {
        let mut distribution_currency: HashMap<String, u32> = HashMap::new();
        for account in self.accounts.iter() {
            distribution_currency
                .entry(account.get_currency().to_string())
                .and_modify(|sum| *sum += account.get_last_amount().unwrap())
                .or_insert(account.get_last_amount().unwrap());
        }

        let mut parts: Vec<PiePiece> = Vec::new();
        let mut total_summ = 0;
        for (currency, summ) in distribution_currency {
            parts.push(PiePiece { size: summ as f64, label: currency });
            total_summ += summ;
        }

        PieChart::create(parts, "Срез по всем балансам в валютах", Some(Self::total_sum_spaced(total_summ)))
    }

    pub fn draw_pie_current_allocations(&self) -> InputFile {
        let account_filtered: Vec<Account> = self.accounts.clone().into_iter()
            .filter(|account| account.get_last_amount().is_some())
            .filter(|account| account.get_last_amount().unwrap() > 0)
            .collect();

        let parts = account_filtered.clone().into_iter().map(|account| {
            let size = account.get_last_amount().unwrap() as f64;
            PiePiece { size, label: account.get_name().clone() }
        }).collect();

        let total_summ: u32 = account_filtered.into_iter().map(|account| account.get_last_amount().unwrap()).sum();

        PieChart::create(parts, "Срез по всем балансам", Some(Self::total_sum_spaced(total_summ)))
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
