use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::asset_type::AssetType::*;

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

impl AssetType {
    pub fn iterator() -> Iter<'static, AssetType> {
        static VALUES: [AssetType; 8] = [
            Cash,
            Crypto,
            Repo,
            Gold,
            Deposit,
            Share,
            Bond,
            BondCurrency,
        ];
        VALUES.iter()
    }
}

impl FromStr for AssetType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cash" => Ok(Cash),
            "crypto" => Ok(Crypto),
            "repo" => Ok(Repo),
            "gold" => Ok(Gold),
            "deposit" => Ok(Deposit),
            "share" => Ok(Share),
            "bond" => Ok(Bond),
            "bondCurrency" => Ok(BondCurrency),
            _ => Err(()),
        }
    }
}
