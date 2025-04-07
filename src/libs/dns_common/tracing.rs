use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TracingConfig {
    pub time: bool,
    pub level: String,
}
