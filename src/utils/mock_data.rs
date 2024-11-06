use crate::db::dao::{Account, Portfolio};
use crate::utils::currency::Currency;

pub struct MockData {}

impl MockData {
    pub fn create() -> Portfolio {
        let mut p = Portfolio::empty();
        let a1 = Account::new("USD".parse().unwrap(), 100, Currency::USD);
        let a2 = Account::new("repo".parse().unwrap(), 500, Currency::RUB);
        let a3 = Account::new("daily".parse().unwrap(), 30, Currency::RUB);
        let a4 = Account::new("broker".parse().unwrap(), 600, Currency::RUB);
        p.add_account(a1);
        p.add_account(a2);
        p.add_account(a3);
        p.add_account(a4);

        p.add_account_record("repo", 550);
        p.add_account_record("repo", 600);
        p.add_account_record("daily", 25);
        p.add_account_record("daily", 15);
        p.add_account_record("daily", 5);
        p.add_account_record("daily", 0);

        p
    }
}