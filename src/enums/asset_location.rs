use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use crate::enums::asset_location::AssetLocation::*;

#[derive(Clone, Debug, Display, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum AssetLocation {
    #[default]
    #[strum(to_string = "Other")]
    Other = 0,
    #[strum(to_string = "Broker1")]
    Broker1 = 1,
    #[strum(to_string = "Broker2")]
    Broker2 = 2,
    #[strum(to_string = "Broker3")]
    Broker3 = 3,
    #[strum(to_string = "Bank1")]
    Bank1 = 4,
    #[strum(to_string = "Bank2")]
    Bank2 = 5,
    #[strum(to_string = "Bank3")]
    Bank3 = 6,
    #[strum(to_string = "PocketMoney")]
    PocketMoney = 7,
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