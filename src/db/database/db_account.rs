use crate::db::account::Account;
use crate::db::database::query::*;
use crate::utils::common::date_to_str;
use rusqlite::{named_params, Connection};
use teloxide::types::ChatId;

type HandlerResult<T> = rusqlite::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait DataBaseAccount {
    fn save(&self, id: ChatId) -> HandlerResult<()>;
    fn save_id(&self, id: i64) -> HandlerResult<()>;
    fn delete(&self) -> HandlerResult<()>;
}

impl DataBaseAccount for Account {
    fn save(&self, id: ChatId) -> HandlerResult<()> {
        Self::save_id(self, id.0)
    }
    fn save_id(&self, id: i64) -> HandlerResult<()> {
        let mut conn = Connection::open(DB_NAME).unwrap();

        let tx = conn.transaction().unwrap();
        tx.execute(
            INSERT_ACCOUNT_SQL,
            named_params! {
            ":id": self.get_id(),
            ":chat_id": id,
            ":name": &self.get_name(),
            ":currency": self.get_currency().clone() as i32,
            ":asset_location": self.get_location() as i32,
            ":asset_type": self.get_type() as i32,
            },
        )?;

        for balance in self.get_balances() {
            tx.execute(
                INSERT_BALANCE_SQL,
                named_params! {
                ":id": balance.get_id(),
                ":account_id": self.get_id(),
                ":amount": &balance.get_amount(),
                ":category": &balance.get_category().map(|c| c as i8).unwrap_or(-1),
                ":date": date_to_str(balance.get_date()),
                },
            )?;
        }
        tx.commit().unwrap();

        Ok(())
    }

    fn delete(&self) -> HandlerResult<()> {
        let conn = Connection::open(DB_NAME).unwrap();

        let mut stmt_delete_account = conn.prepare(DELETE_ACCOUNT_SQL).unwrap();
        stmt_delete_account.execute([self.get_id()]).unwrap();

        Ok(())
    }
}
