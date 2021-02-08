use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationEventData {
    object: Vec<u8>,
    previous_attributes: Option<Vec<u8>>,
}
