use rusqlite::Connection;
use crate::db::database::query::{CREATE_ACCOUNT_SQL, CREATE_BALANCE_SQL, CREATE_PORTFOLIO_SQL, DB_NAME};

type HandlerResult<T> = rusqlite::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct DataBaseTables {}

impl DataBaseTables {
    pub fn create_tables() -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();
        conn.execute(CREATE_PORTFOLIO_SQL, ()).unwrap();
        conn.execute(CREATE_ACCOUNT_SQL, ()).unwrap();
        conn.execute(CREATE_BALANCE_SQL, ()).unwrap();

        Ok(())
    }
}
