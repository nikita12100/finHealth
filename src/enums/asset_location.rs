use strum_macros::Display;

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