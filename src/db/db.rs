use rusqlite::{named_params, Connection};
use teloxide::types::ChatId;
use crate::db::portfolio::Portfolio;

type HandlerResult<T> = rusqlite::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait DataBase {
    fn create_table() -> HandlerResult<()>;
    fn save(&self, id: ChatId) -> HandlerResult<()>;
    fn save_id(&self, id: i64) -> HandlerResult<()>;
    fn get(id: i64) -> Option<Portfolio>;
}

const DB_NAME: &'static str = "portfolios.db";
const CREATE_PORTFOLIO_SQL: &'static str =
    "CREATE TABLE IF NOT EXISTS portfolio (
        id   INTEGER PRIMARY KEY,
        data TEXT,
        accounts TEXT,
        base_currency TEXT NOT NULL DEFAULT \"Rub\",
        exchange_rate TEXT
    )";

const INSERT_PORTFOLIO_SQL: &'static str =
    "INSERT INTO portfolio (id, data, accounts, base_currency, exchange_rate)
     VALUES (:id, :data, :accounts, :base_currency, :exchange_rate) ON CONFLICT (id) DO UPDATE SET
     data=EXCLUDED.data,
     accounts=EXCLUDED.accounts,
     base_currency=EXCLUDED.base_currency,
     exchange_rate=EXCLUDED.exchange_rate
     ";

const SELECT_PORTFOLIO_SQL: &'static str = "SELECT data FROM portfolio where id = :id";


impl DataBase for Portfolio {
    fn create_table() -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();
        conn.execute(CREATE_PORTFOLIO_SQL, ()).unwrap();

        Ok(())
    }

    fn save(&self, id: ChatId) -> HandlerResult<()> { Self::save_id(self, id.0) }
    fn save_id(&self, id: i64) -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();
        let mut stmt = conn.prepare(INSERT_PORTFOLIO_SQL)?;

        stmt.execute(named_params! {
            ":id": id,
            ":data": &serde_json::to_string(&self).unwrap(),
            ":accounts": &serde_json::to_string(self.get_all_accounts()).unwrap(),
            ":base_currency": &serde_json::to_string(self.get_base_currency()).unwrap(),
            ":exchange_rate": &serde_json::to_string(self.get_exchange_rate()).unwrap(),
        })?;

        Ok(())
    }

    fn get(id: i64) -> Option<Portfolio> {
        let conn = Connection::open(DB_NAME).unwrap();

        let mut stmt = conn.prepare(SELECT_PORTFOLIO_SQL).unwrap();
        let mut rows = stmt.query_map(&[(":id", &id.to_string())], |row| row.get(0)).unwrap();

        match rows.next() {
            Some(row) => {
                match row {
                    Ok(row) => {
                        let x: String = row;
                        Some(serde_json::from_str::<Portfolio>(&*x).unwrap())
                    }
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}