use rusqlite::{named_params, Connection};
use teloxide::types::ChatId;
use crate::db::account::Account;
use crate::db::portfolio::Portfolio;
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
        accounts       TEXT,
        base_currency  TEXT NOT NULL DEFAULT \"Rub\",
        exchange_rate  TEXT
    )";

const CREATE_ACCOUNT_SQL: &'static str =
    "CREATE TABLE IF NOT EXISTS account (
        id              INTEGER PRIMARY KEY,
        name            TEXT,
        currency        TEXT NOT NULL DEFAULT \"Rub\",
        asset_location  TEXT,
        asset_type      TEXT,
        balance         TEXT
    )";

const INSERT_PORTFOLIO_SQL: &'static str =
    "INSERT INTO portfolio (id, accounts, base_currency, exchange_rate)
     VALUES (:id, :accounts, :base_currency, :exchange_rate) ON CONFLICT (id) DO UPDATE SET
     accounts=EXCLUDED.accounts,
     base_currency=EXCLUDED.base_currency,
     exchange_rate=EXCLUDED.exchange_rate
     ";

const SELECT_PORTFOLIO_SQL: &'static str = "SELECT accounts, base_currency, exchange_rate FROM portfolio where id = :id";


impl DataBase for Portfolio {
    fn create_tables() -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();
        conn.execute(CREATE_PORTFOLIO_SQL, ()).unwrap();
        conn.execute(CREATE_ACCOUNT_SQL, ()).unwrap();

        Ok(())
    }

    fn save(&self, id: ChatId) -> HandlerResult<()> { Self::save_id(self, id.0) }
    fn save_id(&self, id: i64) -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();
        let mut stmt = conn.prepare(INSERT_PORTFOLIO_SQL)?;

        stmt.execute(named_params! {
            ":id": id,
            ":accounts": &serde_json::to_string(self.get_all_accounts()).unwrap(),
            ":base_currency": &serde_json::to_string(self.get_base_currency()).unwrap(),
            ":exchange_rate": &serde_json::to_string(self.get_exchange_rate()).unwrap(),
        })?;

        Ok(())
    }

    fn get(id: i64) -> Option<Portfolio> {
        let conn = Connection::open(DB_NAME).unwrap();

        let mut stmt = conn.prepare(SELECT_PORTFOLIO_SQL).unwrap();
        let mut portfolios = stmt.query_map(&[(":id", &id.to_string())], |row| {
            Ok(Portfolio::new(
                serde_json::from_str::<Vec<Account>>(&*row.get::<usize, String>(0).unwrap()).unwrap(),
                serde_json::from_str::<Currency>(&*row.get::<usize, String>(1).unwrap()).unwrap(),
                serde_json::from_str::<ExchangeRate>(&*row.get::<usize, String>(2).unwrap()).unwrap(),
            ))
        }).unwrap();

        Option::from(portfolios.next().unwrap().unwrap())
    }
}
