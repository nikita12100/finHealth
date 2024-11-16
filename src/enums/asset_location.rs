use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::asset_location::AssetLocation::*;

#[derive(Clone, Debug, Display, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum AssetLocation {
    #[default]
    #[strum(serialize = "Other", to_string = "Other")]
    Other,
    #[strum(serialize = "Broker1", to_string = "Broker1")]
    Broker1,
    #[strum(serialize = "Broker2", to_string = "Broker2")]
    Broker2,
    #[strum(serialize = "Broker3", to_string = "Broker3")]
    Broker3,
    #[strum(serialize = "Bank1", to_string = "Bank1")]
    Bank1,
    #[strum(serialize = "Bank2", to_string = "Bank2")]
    Bank2,
    #[strum(serialize = "Bank3", to_string = "Bank3")]
    Bank3,
    #[strum(serialize = "PocketMoney", to_string = "PocketMoney")]
    PocketMoney,
}

impl AssetLocation {
    pub fn iterator() -> Iter<'static, AssetLocation> {
        static VALUES: [AssetLocation; 8] = [
            Other,
            Broker1,
            Broker2,
            Broker3,
            Bank1,
            Bank2,
            Bank3,
            PocketMoney,
        ];
        VALUES.iter()
    }
}

impl FromStr for AssetLocation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Other" => Ok(Other),
            "Broker1" => Ok(Broker1),
            "Broker2" => Ok(Broker2),
            "Broker3" => Ok(Broker3),
            "Bank1" => Ok(Bank1),
            "Bank2" => Ok(Bank2),
            "Bank3" => Ok(Bank3),
            "PocketMoney" => Ok(PocketMoney),
            _ => Err(()),
        }
    }
}