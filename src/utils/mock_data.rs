use chrono::{TimeZone, Utc};
use crate::buttons::set_category::Category;
use crate::db::account::Account;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::currency::Currency;

pub struct MockData {}

impl MockData {
    pub fn create() -> Portfolio {
        let mut p = Portfolio::empty();
        let a1 = Account::new_date("$ кеш".parse().unwrap(), 6500, Currency::USD, AssetLocation::PocketMoney, AssetType::Cash, Utc.with_ymd_and_hms(2024, 11, 1, 21, 1, 55).unwrap());
        let a2 = Account::new_date("крипта".parse().unwrap(), 200000, Currency::RUB, AssetLocation::Broker2, AssetType::Crypto, Utc.with_ymd_and_hms(2024, 11, 2, 21, 1, 55).unwrap());
        let a33 = Account::new_date("TMON".parse().unwrap(), 100000, Currency::RUB, AssetLocation::Broker1, AssetType::Repo, Utc.with_ymd_and_hms(2024, 11, 1, 22, 1, 55).unwrap());
        let a3 = Account::new_date("TPAY".parse().unwrap(), 303000, Currency::RUB, AssetLocation::Broker1, AssetType::Bond, Utc.with_ymd_and_hms(2024, 11, 1, 22, 1, 55).unwrap());
        let a4 = Account::new_date("металл".parse().unwrap(), 115779, Currency::RUB, AssetLocation::Broker2, AssetType::Gold, Utc.with_ymd_and_hms(2024, 11, 1, 21, 17, 55).unwrap());
        let a5 = Account::new_date("вклад".parse().unwrap(), 55000, Currency::RUB, AssetLocation::Bank1, AssetType::Deposit, Utc.with_ymd_and_hms(2024, 11, 1, 23, 1, 55).unwrap());
        let a6 = Account::new_date("акции".parse().unwrap(), 474512, Currency::RUB, AssetLocation::PocketMoney, AssetType::Share, Utc.with_ymd_and_hms(2024, 11, 1, 21, 1, 55).unwrap());
        let a7 = Account::new_date("обилиги руб".parse().unwrap(), 307555, Currency::RUB, AssetLocation::Broker2, AssetType::Bond, Utc.with_ymd_and_hms(2024, 11, 1, 21, 15, 55).unwrap());
        let a8 = Account::new_date("обилиги $".parse().unwrap(), 2389, Currency::USD, AssetLocation::Broker2, AssetType::BondCurrency, Utc.with_ymd_and_hms(2024, 11, 1, 21, 1, 55).unwrap());
        let a9 = Account::new_date("daily".parse().unwrap(), 100000, Currency::RUB, AssetLocation::Bank1, AssetType::Cash, Utc.with_ymd_and_hms(2024, 11, 1, 21, 29, 55).unwrap());
        p.add_account(a1);
        p.add_account(a2);
        p.add_account(a3);
        p.add_account(a33);
        p.add_account(a4);
        p.add_account(a5);
        p.add_account(a6);
        p.add_account(a7);
        p.add_account(a8);
        p.add_account(a9);

        p.get_account_mut("крипта").unwrap().add_balance_income_date(50000, Utc.with_ymd_and_hms(2024, 11, 3, 21, 1, 55).unwrap());
        p.get_account_mut("TMON").unwrap().add_balance_income_date(5000, Utc.with_ymd_and_hms(2024, 11, 3, 21, 1, 55).unwrap());
        p.get_account_mut("TPAY").unwrap().add_balance_income_date(100500, Utc.with_ymd_and_hms(2024, 11, 3, 21, 1, 55).unwrap());
        p.get_account_mut("металл").unwrap().add_balance_income_date(15000, Utc.with_ymd_and_hms(2024, 11, 3, 21, 1, 55).unwrap());
        p.get_account_mut("вклад").unwrap().add_balance_outcome_date(15000, Category::OTHER.to_string(), Utc.with_ymd_and_hms(2024, 11, 3, 21, 1, 55).unwrap());

        p.get_account_mut("daily").unwrap().add_balance_outcome_date(10000, Category::TAXI.to_string(), Utc.with_ymd_and_hms(2024, 11, 2, 22, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(5000, Category::PRODUCTS.to_string(), Utc.with_ymd_and_hms(2024, 11, 3, 21, 15, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(30000, Category::APARTMENT_RENT.to_string(), Utc.with_ymd_and_hms(2024, 11, 5, 23, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(5000, Category::APARTMENT_RENT.to_string(), Utc.with_ymd_and_hms(2024, 11, 6, 21, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(4000, Category::CAFES_AND_RESTAURANTS.to_string(), Utc.with_ymd_and_hms(2024, 11, 8, 21, 19, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(20000, Category::PETS.to_string(), Utc.with_ymd_and_hms(2024, 11, 10, 22, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_income_date(40000, Utc.with_ymd_and_hms(2024, 11, 11, 12, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(5000, Category::APARTMENT_RENT.to_string(), Utc.with_ymd_and_hms(2024, 11, 11, 15, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(30000, Category::PRODUCTS.to_string(), Utc.with_ymd_and_hms(2024, 11, 11, 20, 1, 55).unwrap());

        p
    }
}