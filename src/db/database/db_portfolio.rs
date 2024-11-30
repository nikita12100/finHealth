use crate::db::account::Account;
use crate::db::balance_timed::BalanceTimed;
use crate::db::database::query::*;
use crate::db::portfolio::Portfolio;
use crate::utils::common::{date_to_str, str_to_date};
use crate::utils::exchange_rate::ExchangeRate;
use rusqlite::{named_params, Connection};
use teloxide::types::ChatId;

type HandlerResult<T> = rusqlite::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait DataBasePortfolio {
    fn save(&self, id: ChatId) -> HandlerResult<()>;
    fn save_id(&self, id: i64) -> HandlerResult<()>;
    fn get(id: i64) -> Option<Portfolio>;
}

impl DataBasePortfolio for Portfolio {
    fn save(&self, id: ChatId) -> HandlerResult<()> {
        Self::save_id(self, id.0)
    }
    fn save_id(&self, id: i64) -> HandlerResult<()> {
        let mut conn = Connection::open(DB_NAME).unwrap();

        let tx = conn.transaction().unwrap();
        tx.execute(
            INSERT_PORTFOLIO_SQL,
            named_params! {
            ":id": id,
            ":base_currency": self.get_base_currency().clone() as i32,
            ":exchange_rate": &serde_json::to_string(self.get_exchange_rate()).unwrap(),
            },
        )?;

        for account in self.get_all_accounts() {
            tx.execute(
                INSERT_ACCOUNT_SQL,
                named_params! {
                ":id": account.get_id(),
                ":chat_id": id,
                ":name": &account.get_name(),
                ":currency": account.get_currency().clone() as i32,
                ":asset_location": account.get_location() as i32,
                ":asset_type": account.get_type() as i32,
                },
            )?;

            for balance in account.get_balances() {
                tx.execute(
                    INSERT_BALANCE_SQL,
                    named_params! {
                    ":id": balance.get_id(),
                    ":account_id": account.get_id(),
                    ":amount": &balance.get_amount(),
                    ":category": &balance.get_category().map(|c| c as i8).unwrap_or(-1),
                    ":date": date_to_str(balance.get_date()),
                    },
                )?;
            }
        }
        tx.commit().unwrap();

        Ok(())
    }

    fn get(id: i64) -> Option<Portfolio> {
        let conn = Connection::open(DB_NAME).unwrap();

        let mut stmt_accounts = conn.prepare(SELECT_ACCOUNT_SQL).unwrap();
        let accounts_rows = stmt_accounts
            .query_map(&[(":chat_id", &id.to_string())], |row| {
                Ok(Account::load_db(
                    row.get::<usize, String>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, i32>(2).unwrap().try_into().unwrap(),
                    row.get::<usize, i32>(3).unwrap().try_into().unwrap(),
                    row.get::<usize, i32>(4).unwrap().try_into().unwrap(),
                    Vec::new(),
                ))
            })
            .unwrap();
        let mut accounts = accounts_rows.map(|c| c.unwrap()).collect::<Vec<_>>();

        for account in &mut accounts {
            let mut stmt_balances = conn.prepare(SELECT_BALANCE_SQL).unwrap();
            let balances_rows = stmt_balances
                .query_map(&[(":account_id", &account.get_id())], |row| {
                    Ok(BalanceTimed::load_db(
                        row.get::<usize, String>(0).unwrap(),
                        row.get::<usize, u32>(1).unwrap(),
                        row.get::<usize, i32>(2).unwrap_or(-1).try_into().ok(),
                        str_to_date(row.get::<usize, String>(3).unwrap()),
                    ))
                })
                .unwrap();
            let balances = balances_rows.map(|c| c.unwrap()).collect::<Vec<_>>();

            account.append_balance_db(balances)
        }

        let mut stmt_portfolio = conn.prepare(SELECT_PORTFOLIO_SQL).unwrap();
        let mut portfolios = stmt_portfolio
            .query_map(&[(":id", &id.to_string())], |row| {
                Ok(Portfolio::new(
                    accounts.clone(),
                    row.get::<usize, i32>(0).unwrap().try_into().unwrap(),
                    serde_json::from_str::<ExchangeRate>(&*row.get::<usize, String>(1).unwrap())
                        .unwrap(),
                ))
            })
            .unwrap();

        portfolios.next().map(|c| c.ok()).flatten()
    }
}
