use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::asset_type::AssetType::*;

#[derive(Clone, Debug, Display, Default, PartialEq)]
pub enum AssetType {
    #[default]
    #[strum(to_string = "cash")]
    Cash = 0,
    #[strum(to_string = "crypto")]
    Crypto = 1,
    #[strum(to_string = "repo")]
    Repo = 2,
    #[strum(to_string = "gold")]
    Gold = 3,
    #[strum(to_string = "deposit")]
    Deposit = 4,
    #[strum(to_string = "share")]
    Share = 5,
    #[strum(to_string = "bond")]
    Bond = 6,
    #[strum(to_string = "bond$")]
    BondCurrency = 7,
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

impl TryFrom<i32> for AssetType {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Cash as i32 => Ok(Cash),
            x if x == Crypto as i32 => Ok(Crypto),
            x if x == Repo as i32 => Ok(Repo),
            x if x == Gold as i32 => Ok(Gold),
            x if x == Deposit as i32 => Ok(Deposit),
            x if x == Share as i32 => Ok(Share),
            x if x == Bond as i32 => Ok(Bond),
            x if x == BondCurrency as i32 => Ok(BondCurrency),
            _ => Err(()),
        }
    }
}
