pub(crate) const DB_NAME: &'static str = "portfolios.db";

// ========================================== <TABLES> ==========================================
pub(crate) const CREATE_PORTFOLIO_SQL: &'static str = "CREATE TABLE IF NOT EXISTS portfolio (
        id             INTEGER PRIMARY KEY,
        base_currency  INTEGER NOT NULL DEFAULT 0,
        exchange_rate  TEXT
    )";

pub(crate) const CREATE_ACCOUNT_SQL: &'static str = "CREATE TABLE IF NOT EXISTS account (
        id              TEXT PRIMARY KEY,
        chat_id         INTEGER NOT NULL,
        name            TEXT,
        currency        INTEGER NOT NULL DEFAULT 0,
        asset_location  INTEGER NOT NULL DEFAULT 0,
        asset_type      INTEGER NOT NULL DEFAULT 0
    )";

pub(crate) const CREATE_BALANCE_SQL: &'static str = "CREATE TABLE IF NOT EXISTS balance (
        id              TEXT PRIMARY KEY,
        account_id      TEXT NOT NULL,
        amount          INTEGER NOT NULL,
        category        INTEGER NOT NULL DEFAULT -1,
        date            TEXT NOT NULL
    )";

// ========================================== >TABLES< ==========================================

pub(crate) const INSERT_PORTFOLIO_SQL: &'static str =
    "INSERT INTO portfolio (id, base_currency, exchange_rate)
     VALUES (:id, :base_currency, :exchange_rate) ON CONFLICT (id) DO UPDATE SET
     base_currency=EXCLUDED.base_currency,
     exchange_rate=EXCLUDED.exchange_rate
     ";

pub(crate) const INSERT_ACCOUNT_SQL: &'static str =
    "INSERT INTO account (id, chat_id, name, currency, asset_location, asset_type)
     VALUES (:id, :chat_id, :name, :currency, :asset_location, :asset_type) ON CONFLICT (id) DO UPDATE SET
     chat_id=EXCLUDED.chat_id,
     name=EXCLUDED.name,
     currency=EXCLUDED.currency,
     asset_location=EXCLUDED.asset_location,
     asset_type=EXCLUDED.asset_type
     ";

pub(crate) const INSERT_BALANCE_SQL: &'static str =
    "INSERT INTO balance (id, account_id, amount, category, date)
     VALUES (:id, :account_id, :amount, :category, :date) ON CONFLICT (id) DO UPDATE SET
     account_id=EXCLUDED.account_id,
     amount=EXCLUDED.amount,
     category=EXCLUDED.category,
     date=EXCLUDED.date
     ";

pub(crate) const SELECT_PORTFOLIO_SQL: &'static str =
    "SELECT base_currency, exchange_rate FROM portfolio WHERE id = :id";
pub(crate) const SELECT_ACCOUNT_SQL: &'static str =
    "SELECT id, name, currency, asset_location, asset_type FROM account where chat_id = :chat_id";
pub(crate) const SELECT_BALANCE_SQL: &'static str =
    "SELECT id, amount, category, date FROM balance where account_id = :account_id";

pub(crate) const DELETE_ACCOUNT_SQL: &'static str = "DELETE FROM account where id = :id";
