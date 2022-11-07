use serde::Deserializer;
use std::fmt::{Display, Formatter};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Party {
    pub id: u32,
    pub name: String,
    pub gift: GiftType,
    pub email: String,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub(crate) enum GiftType {
    None,
    Cash,
    Check,
    #[serde(rename = "E-Transfer")]
    ETransfer,
    #[serde(rename = "Gift Card")]
    GiftCard,
    Other(String),
}

impl Display for GiftType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GiftType::None => "",
            GiftType::Cash => "cash",
            GiftType::Check => "check",
            GiftType::ETransfer => "e-transfer",
            GiftType::GiftCard => "gift card",
            GiftType::Other(inner) => inner,
        };
        write!(f, "{}", str)
    }
}

struct StringVisitor;

impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = GiftType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v.to_ascii_lowercase().as_str() {
            "none" => GiftType::None,
            "cash" => GiftType::Cash,
            "check" => GiftType::Check,
            "e-transfer" => GiftType::ETransfer,
            "gift card" => GiftType::GiftCard,
            _ => GiftType::Other(v.to_string()),
        })
    }
}

impl<'de> serde::Deserialize<'de> for GiftType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor)
    }
}
