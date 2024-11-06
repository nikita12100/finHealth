use std::str::FromStr;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Currency {
    #[default]
    RUB,
    USD,
}

impl FromStr for Currency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rub" => Ok(Currency::RUB),
            "usd" => Ok(Currency::USD),
            _ => Err(()),
        }
    }
}
