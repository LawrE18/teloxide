use serde::{Deserialize, Serialize};

use crate::types::WebAppInfo;

/// Represents a button to be shown above inline query results.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultsbutton).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
    /// Label text on the button.
    pub text: String,
    /// Description of the Web App that will be launched when the user presses
    /// the button. The Web App will be able to switch back to the inline
    /// mode using the method switchInlineQuery inside the Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// Deep-linking parameter for the /start message sent to the bot when a
    /// user presses the button. 1-64 characters, only A-Z, a-z, 0-9, _ and
    /// - are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}
