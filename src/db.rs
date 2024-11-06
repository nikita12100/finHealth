use rusqlite::{named_params, Connection};
use teloxide::prelude::ChatId;
use crate::dao::Portfolio;

type HandlerResult<T> = rusqlite::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait DataBase {
    fn create_table() -> HandlerResult<()>;
    fn save(&self, id: i64) -> HandlerResult<()>;
    fn get(id: i64) -> HandlerResult<Portfolio>;
}

impl DataBase for Portfolio {
    fn create_table() -> HandlerResult<()> {
        let conn = Connection::open("portfolios.db").unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS portfolio (
            id   INTEGER PRIMARY KEY,
            data TEXT
        )",
            (), // empty list of parameters.
        ).unwrap();

        Ok(())
    }

    fn save(&self, id: i64) -> HandlerResult<()> {
        let conn = Connection::open("portfolios.db").unwrap();
        let mut stmt = conn.prepare("INSERT INTO portfolio (id, data) VALUES (:id, :data) ON CONFLICT (id) DO UPDATE SET data=EXCLUDED.data")?;

        stmt.execute(named_params! {":id": id, ":data": &serde_json::to_string(&self).unwrap()})?;

        Ok(())
    }

    fn get(id: i64) -> HandlerResult<Portfolio> {
        let conn = Connection::open("portfolios.db")?;

        let mut stmt = conn.prepare("SELECT data FROM portfolio where id = :id")?;
        let mut rows = stmt.query_map(&[(":id", &id.to_string())], |row| row.get(0))?;

        match rows.next() {
            Some(row) => {
                match row {
                    Ok(row) => {
                        let x: String = row;
                        Ok(serde_json::from_str::<Portfolio>(&*x).unwrap())
                    }
                    Err(err) => Err(err.into()),
                }
            }
            None => Err("no rows returned".into()),
        }
    }
}