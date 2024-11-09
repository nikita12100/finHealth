use teloxide::types::InputFile;
use crate::charts::pie_chart::{PieChart, PiePiece};
use crate::db::account::Account;

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

    pub fn draw_pie_current_allocations(&self) -> InputFile {
        let parts = self.accounts.iter()
            .filter(|account| account.get_last_amount().is_some())
            .filter(|account| account.get_last_amount().unwrap() > 0)
            .map(|account| {
                let size = account.get_last_amount().unwrap() as f64;
                PiePiece { size, label: account.get_name().clone() }
            }).collect();

        let total_summ: u32 = self.accounts.iter()
            .filter(|account| account.get_last_amount().is_some())
            .filter(|account| account.get_last_amount().unwrap() > 0)
            .map(|account| account.get_last_amount().unwrap())
            .sum();

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

        PieChart::create(parts, "Срез по всем балансам", Some(total_sum_str.iter().collect()))
    }
}
