use serde::de::{self, DeserializeOwned, Deserializer};
use serde::Deserialize;

pub(crate) fn double_serialized<'de, V, D>(deserializer: D) -> Result<V, D::Error>
where
    V: DeserializeOwned,
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    serde_json::from_str(&buf).map_err(de::Error::custom)
}

pub(crate) trait EventHubPayload {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "")]
pub(crate) struct EventHubTrigger<T>
where
    T: DeserializeOwned,
{
    pub(crate) data: EventHubData<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EventHubData<T>
where
    T: DeserializeOwned,
{
    #[serde(deserialize_with = "double_serialized")]
    pub(crate) event_hub_messages: Vec<T>,
}
