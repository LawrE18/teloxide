use serde::{Deserialize, Serialize};

use crate::types::PaymentsForm;

/// This object represents a custom query.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomQuery {
    /// Unique custom query identifier.
    pub id: String,

    /// Custom query method.
    pub method: CustomQueryMethod,

    /// Custom query kind.
    #[serde(flatten)]
    pub kind: CustomQueryKind,
}

/// Custom query methods.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CustomQueryMethod {
    #[serde(rename = "payments.form")]
    PaymentsForm,
}

/// Supported custom query kinds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomQueryKind {
    PaymentsForm(PaymentsForm),
}
