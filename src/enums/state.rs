#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    // Listen buttons click
    ListenStartButtonsCallback,
    ListenGetPortfolioButtonsCallback,
    ListenEditPortfolioButtonsCallback,
    ListenSetBaseCurrencyButtonsCallback,
    ListenCategoryCallback {
        account_name: String,
        outcome: u32,
    },
    // Listen client data from chat
    ListenBalanceNameCallback,
    ListenNewAccountName,
    ListenAccountIncomeFor(String),
    ListenAccountOutcomeFor(String),
    ListenCurrencyForCallback(String),
    ListenLocationForCallback(String),
    ListenTypeForCallback(String),
    // Get client data from chat for each listen
    GotListenAccountNameListenAccountButtonsCallback(String),
    GotNewAccountName(String),
}
