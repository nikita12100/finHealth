use rusqlite::{named_params, Connection};
use teloxide::types::ChatId;
use crate::db::account::Account;
use crate::db::balance_timed::BalanceTimed;
use crate::db::portfolio::Portfolio;
use crate::enums::asset_location::AssetLocation;
use crate::enums::asset_type::AssetType;
use crate::enums::currency::Currency;
use crate::utils::exchange_rate::ExchangeRate;

type HandlerResult<T> = rusqlite::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait DataBase {
    fn create_tables() -> HandlerResult<()>;
    fn save(&self, id: ChatId) -> HandlerResult<()>;
    fn save_id(&self, id: i64) -> HandlerResult<()>;
    fn get(id: i64) -> Option<Portfolio>;
}

const DB_NAME: &'static str = "portfolios.db";
const CREATE_PORTFOLIO_SQL: &'static str =
    "CREATE TABLE IF NOT EXISTS portfolio (
        id             INTEGER PRIMARY KEY,
        base_currency  INTEGER NOT NULL DEFAULT 0,
        exchange_rate  TEXT
    )";

const CREATE_ACCOUNT_SQL: &'static str =
    "CREATE TABLE IF NOT EXISTS account (
        id              TEXT PRIMARY KEY,
        chat_id         INTEGER NOT NULL DEFAULT 0,
        name            TEXT,
        currency        INTEGER NOT NULL DEFAULT 0,
        asset_location  INTEGER NOT NULL DEFAULT 0,
        asset_type      INTEGER NOT NULL DEFAULT 0,
        balance         TEXT
    )";

const INSERT_PORTFOLIO_SQL: &'static str =
    "INSERT INTO portfolio (id, base_currency, exchange_rate)
     VALUES (:id, :base_currency, :exchange_rate) ON CONFLICT (id) DO UPDATE SET
     base_currency=EXCLUDED.base_currency,
     exchange_rate=EXCLUDED.exchange_rate
     ";

const INSERT_ACCOUNT_SQL: &'static str =
    "INSERT INTO account (id, chat_id, name, currency, asset_location, asset_type, balance)
     VALUES (:id, :chat_id, :name, :currency, :asset_location, :asset_type, :balance) ON CONFLICT (id) DO UPDATE SET
     chat_id=EXCLUDED.chat_id,
     name=EXCLUDED.name,
     currency=EXCLUDED.currency,
     asset_location=EXCLUDED.asset_location,
     asset_type=EXCLUDED.asset_type,
     balance=EXCLUDED.balance
     ";

const SELECT_PORTFOLIO_SQL: &'static str = "SELECT base_currency, exchange_rate FROM portfolio WHERE id = :id";
const SELECT_ACCOUNT_SQL: &'static str = "SELECT id, name, currency, asset_location, asset_type, balance FROM account where chat_id = :chat_id";


impl DataBase for Portfolio {
    fn create_tables() -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();
        conn.execute("DROP TABLE IF EXISTS portfolio", [])?; // todo remove
        conn.execute("DROP TABLE IF EXISTS account", [])?;
        conn.execute(CREATE_PORTFOLIO_SQL, ()).unwrap();
        conn.execute(CREATE_ACCOUNT_SQL, ()).unwrap();

        Ok(())
    }

    fn save(&self, id: ChatId) -> HandlerResult<()> { Self::save_id(self, id.0) }
    fn save_id(&self, id: i64) -> HandlerResult<()> {
        let mut conn = Connection::open(DB_NAME).unwrap();

        let tx = conn.transaction().unwrap();
        tx.execute(INSERT_PORTFOLIO_SQL, named_params! {
            ":id": id,
            ":base_currency": &serde_json::to_string(self.get_base_currency()).unwrap(),
            ":exchange_rate": &serde_json::to_string(self.get_exchange_rate()).unwrap(),
            })?;

        for account in self.get_all_accounts() {
            tx.execute(INSERT_ACCOUNT_SQL, named_params! {
                ":id": account.get_id().to_string(),
                ":chat_id": id,
                ":name": &account.get_name(),
                ":currency": &serde_json::to_string(account.get_currency()).unwrap(),
                ":asset_location": &serde_json::to_string(&account.get_location()).unwrap(),
                ":asset_type": &serde_json::to_string(&account.get_type()).unwrap(),
                ":balance": &serde_json::to_string(&account.get_balances()).unwrap(),
                })?;
        }
        tx.commit().unwrap();


        Ok(())
    }

    fn get(id: i64) -> Option<Portfolio> {
        let conn = Connection::open(DB_NAME).unwrap();

        let mut stmt2 = conn.prepare(SELECT_ACCOUNT_SQL).unwrap();
        let accounts = stmt2.query_map(&[(":chat_id", &id.to_string())], |row| {
            Ok(Account::load_db(
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
                serde_json::from_str::<Currency>(&*row.get::<usize, u8>(2).unwrap().to_string()).unwrap(),
                serde_json::from_str::<AssetLocation>(&*row.get::<usize, u8>(3).unwrap().to_string()).unwrap(),
                serde_json::from_str::<AssetType>(&*row.get::<usize, u8>(4).unwrap().to_string()).unwrap(),
                serde_json::from_str::<Vec<BalanceTimed>>(&*row.get::<usize, String>(5).unwrap().to_string()).unwrap(),
            ))
        }).unwrap();

        let x = accounts.map(|c|c.unwrap()).collect::<Vec<_>>();

        let mut stmt = conn.prepare(SELECT_PORTFOLIO_SQL).unwrap();
        let mut portfolios = stmt.query_map(&[(":id", &id.to_string())], |row| {
            Ok(Portfolio::new(
                x.clone(),
                serde_json::from_str::<Currency>(&*row.get::<usize, u8>(0).unwrap().to_string()).unwrap(),
                serde_json::from_str::<ExchangeRate>(&*row.get::<usize, String>(1).unwrap()).unwrap(),
            ))
        }).unwrap();

        Option::from(portfolios.next().unwrap().unwrap())
    }
}
