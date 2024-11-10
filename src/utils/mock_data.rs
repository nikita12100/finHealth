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
        let a1 = Account::new("$ кеш".parse().unwrap(), 6500, Currency::USD, AssetLocation::PocketMoney, AssetType::Cash);
        let a2 = Account::new("крипта".parse().unwrap(), 250000, Currency::RUB, AssetLocation::Broker2, AssetType::Crypto);
        let a3 = Account::new("TMON".parse().unwrap(), 871995, Currency::RUB, AssetLocation::Broker1, AssetType::Repo);
        let a4 = Account::new("металл".parse().unwrap(), 66806, Currency::RUB, AssetLocation::Broker2, AssetType::Gold);
        let a5 = Account::new("вклад".parse().unwrap(), 55000, Currency::RUB, AssetLocation::Bank1, AssetType::Deposit);
        let a6 = Account::new("акции".parse().unwrap(), 326687, Currency::RUB, AssetLocation::PocketMoney, AssetType::Share);
        let a7 = Account::new("обилиги руб".parse().unwrap(), 216554, Currency::RUB, AssetLocation::Broker2, AssetType::Bond);
        let a8 = Account::new("обилиги $".parse().unwrap(), 1585, Currency::USD, AssetLocation::Broker2, AssetType::BondCurrency);
        let a9 = Account::new("daily".parse().unwrap(), 100000, Currency::RUB, AssetLocation::Bank1, AssetType::Cash);
        p.add_account(a1);
        p.add_account(a2);
        p.add_account(a3);
        p.add_account(a4);
        p.add_account(a5);
        p.add_account(a6);
        p.add_account(a7);
        p.add_account(a8);
        p.add_account(a9);

        p.get_account_mut("daily").unwrap().add_balance_outcome_date(25000, Category::PRODUCTS.to_string(), Utc.with_ymd_and_hms(2024, 11, 1, 21, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(10000, Category::TAXI.to_string(), Utc.with_ymd_and_hms(2024, 11, 2, 22, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(5000, Category::PRODUCTS.to_string(), Utc.with_ymd_and_hms(2024, 11, 3, 21, 15, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(30000, Category::APARTMENT_RENT.to_string(), Utc.with_ymd_and_hms(2024, 11, 5, 23, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(5000, Category::APARTMENT_RENT.to_string(), Utc.with_ymd_and_hms(2024, 11, 6, 21, 1, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(4000, Category::CAFES_AND_RESTAURANTS.to_string(), Utc.with_ymd_and_hms(2024, 11, 8, 21, 19, 55).unwrap());
        p.get_account_mut("daily").unwrap().add_balance_outcome_date(20000, Category::PETS.to_string(), Utc.with_ymd_and_hms(2024, 11, 10, 22, 1, 55).unwrap());

        p
    }
}