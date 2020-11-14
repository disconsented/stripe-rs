// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;

/// The resource representing a Stripe "TerminalConnectionToken".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}

impl Object for TerminalConnectionToken {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "terminal.connection_token"
    }
}
