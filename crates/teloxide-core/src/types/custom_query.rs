use serde::{Deserialize, Serialize};

use crate::types::CustomQueryId;

/// This object represents a message.
///
/// [The official docs](https://core.telegram.org/bots/api#message).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomQuery {
    /// Unique custom query identifier.
    #[serde(flatten)]
    pub id: CustomQueryId,

    /// Custom query kind.
    #[serde(flatten)]
    pub kind: CustomQueryKind,
}

/// Custom query kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomQueryKind {
    PaymentsForm(PaymentsForm),
}

/// This object represents a payment.form handled webhook.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentsForm {
    /// Customer id.
    pub customer_id: i32,

    /// Optional Customer's IETF language tag.
    pub customer_language_code: Option<String>,

    /// Merchant bot id in Telegram Api.
    pub bot_id: i32,

    /// Merchant bot connected account.
    pub bot_account: String,

    /// Merchant bot username.
    pub bot_username: String,

    /// Merchant owner id.
    pub bot_owner_id: i32,

    /// Optional (if the CUSTOMER’s app supports it) color schema variables
    /// to be used in the form. Provider can use it to adapt design of credit
    /// card form to user’s current theme.
    pub theme_params: Option<ThemeParams>,
}

/// Theme params.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeParams {
    /// Bg color.
    pub bg_color: String,
    /// Text color.
    pub text_color: String,
    /// Hint color.
    pub hint_color: String,
    /// Link color.
    pub link_color: String,
    /// Button color.
    pub button_color: String,
    /// Button text color.
    pub button_text_color: String,
}
