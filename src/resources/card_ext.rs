use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CheckResult {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Failed,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unchecked")]
    Unchecked,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CardBrand {
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Diners Club")]
    DinersClub,
    #[serde(rename = "Discover")]
    Discover,
    #[serde(rename = "JCB")]
    JCB,
    #[serde(rename = "Visa")]
    Visa,
    #[serde(rename = "MasterCard")]
    MasterCard,
    #[serde(rename = "UnionPay")]
    UnionPay,

    /// An unknown card brand.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "Unknown")]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "prepaid")]
    Prepaid,

    /// An unknown card type.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenizationMethod {
    ApplePay,
    AndroidPay,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}