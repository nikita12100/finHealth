use crate::enums::asset_location::AssetLocation::*;
use std::slice::Iter;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Clone, Debug, Default, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum AssetLocation {
    #[default]
    #[strum(serialize = "Other")]
    Other = 0,
    #[strum(serialize = "Broker1")]
    Broker1 = 1,
    #[strum(serialize = "Broker2")]
    Broker2 = 2,
    #[strum(serialize = "Broker3")]
    Broker3 = 3,
    #[strum(serialize = "Bank1")]
    Bank1 = 4,
    #[strum(serialize = "Bank2")]
    Bank2 = 5,
    #[strum(serialize = "Bank3")]
    Bank3 = 6,
    #[strum(serialize = "PocketMoney")]
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

impl TryFrom<i32> for AssetLocation {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Other as i32 => Ok(Other),
            x if x == Broker1 as i32 => Ok(Broker1),
            x if x == Broker2 as i32 => Ok(Broker2),
            x if x == Broker3 as i32 => Ok(Broker3),
            x if x == Bank1 as i32 => Ok(Bank1),
            x if x == Bank2 as i32 => Ok(Bank2),
            x if x == Bank3 as i32 => Ok(Bank3),
            x if x == PocketMoney as i32 => Ok(PocketMoney),
            _ => Err(()),
        }
    }
}
