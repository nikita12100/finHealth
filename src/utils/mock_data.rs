use crate::db::account::Account;
use crate::db::portfolio::Portfolio;
use crate::utils::currency::Currency;

pub struct MockData {}

impl MockData {
    pub fn create() -> Portfolio {
        let mut p = Portfolio::empty();
        let a1 = Account::new("USD".parse().unwrap(), 300, Currency::USD);
        let a2 = Account::new("repo".parse().unwrap(), 500, Currency::RUB);
        let a3 = Account::new("daily".parse().unwrap(), 30, Currency::RUB);
        let a4 = Account::new("broker".parse().unwrap(), 600, Currency::RUB);
        p.add_account(a1);
        p.add_account(a2);
        p.add_account(a3);
        p.add_account(a4);

        p.get_account_mut("repo").unwrap().set_balance_amount(550);
        p.get_account_mut("repo").unwrap().set_balance_amount(600);
        p.get_account_mut("daily").unwrap().set_balance_amount(25);
        p.get_account_mut("daily").unwrap().set_balance_amount(15);
        p.get_account_mut("daily").unwrap().set_balance_amount(5);
        p.get_account_mut("daily").unwrap().set_balance_amount(0);

        p
    }
}