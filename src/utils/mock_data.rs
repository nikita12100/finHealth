use crate::db::account::Account;
use crate::db::portfolio::Portfolio;
use crate::utils::currency::Currency;

pub struct MockData {}

impl MockData {
    pub fn create() -> Portfolio {
        let mut p = Portfolio::empty();
        // let a1 = Account::new("доллары".parse().unwrap(), 6500, Currency::USD);
        let a2 = Account::new("доллары р.".parse().unwrap(), 585000, Currency::USD);
        let a3 = Account::new("Копилка".parse().unwrap(), 551073, Currency::RUB);
        let a4 = Account::new("крипта".parse().unwrap(), 250000, Currency::RUB);
        let a5 = Account::new("брок счет".parse().unwrap(), 520073, Currency::RUB);
        let a6 = Account::new("вклад".parse().unwrap(), 409141, Currency::RUB);
        let a7 = Account::new("daily".parse().unwrap(), 32000, Currency::RUB);
        // p.add_account(a1);
        p.add_account(a2);
        p.add_account(a3);
        p.add_account(a4);
        p.add_account(a5);
        p.add_account(a6);
        p.add_account(a7);

        // p.get_account_mut("repo").unwrap().set_balance_amount(550);
        // p.get_account_mut("repo").unwrap().set_balance_amount(600);

        p
    }
}