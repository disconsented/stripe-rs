use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationEventData {
    pub object: Vec<u8>,
    pub previous_attributes: Option<Vec<u8>>,
}
