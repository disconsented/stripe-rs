use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationEventData {
    pub object: Value,
    pub previous_attributes: Option<Value>,
}
