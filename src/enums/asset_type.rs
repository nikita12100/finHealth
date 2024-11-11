use strum_macros::Display;

#[derive(Clone, Debug, Display, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum AssetType {
    #[default]
    #[strum(serialize = "cash", to_string = "cash")]
    Cash,
    #[strum(serialize = "crypto", to_string = "crypto")]
    Crypto,
    #[strum(serialize = "repo", to_string = "repo")]
    Repo,
    #[strum(serialize = "gold", to_string = "gold")]
    Gold,
    #[strum(serialize = "deposit", to_string = "deposit")]
    Deposit,
    #[strum(serialize = "share", to_string = "share")]
    Share,
    #[strum(serialize = "bond", to_string = "bond")]
    Bond,
    #[strum(serialize = "bond$", to_string = "bond$")]
    BondCurrency,
}