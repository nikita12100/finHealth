use crate::enums::asset_type::AssetType::*;
use std::slice::Iter;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Clone, Debug, Default, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum AssetType {
    #[default]
    #[strum(serialize = "💵 cash")]
    Cash = 0,
    #[strum(serialize = "₿ crypto")]
    Crypto = 1,
    #[strum(serialize = "repo")]
    Repo = 2,
    #[strum(serialize = "🧈 gold")]
    Gold = 3,
    #[strum(serialize = "🏦 deposit")]
    Deposit = 4,
    #[strum(serialize = "🚀 share")]
    Share = 5,
    #[strum(serialize = "bond")]
    Bond = 6,
    #[strum(serialize = "bond$")]
    BondCurrency = 7,
}

impl AssetType {
    pub fn iterator() -> Iter<'static, AssetType> {
        static VALUES: [AssetType; 8] =
            [Cash, Crypto, Repo, Gold, Deposit, Share, Bond, BondCurrency];
        VALUES.iter()
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
