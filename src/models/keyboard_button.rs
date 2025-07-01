//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-01T14:36:16.092164073Z[Etc/UTC]
//! - **Generator Version**: 7.14.0
//!
//! <details>
//! <summary><strong>⚠️ Important Disclaimer & Limitation of Liability</strong></summary>
//! <br>
//! > **IMPORTANT**: This software is provided "as is" without any warranties, express or implied, including but not limited
//! > to warranties of merchantability, fitness for a particular purpose, or non-infringement. The developers, contributors,
//! > and licensors (collectively, "Developers") make no representations regarding the accuracy, completeness, or reliability
//! > of this software or its outputs.
//! >
//! > This client is not intended to provide financial, investment, tax, or legal advice. It facilitates interaction with the
//! > Telegram Bot API service but does not endorse or recommend any financial actions, including the purchase, sale, or holding of
//! > financial instruments (e.g., stocks, bonds, derivatives, cryptocurrencies). Users must consult qualified financial or
//! > legal professionals before making decisions based on this software's outputs.
//! >
//! > Financial markets are inherently speculative and carry significant risks. Using this software in trading, analysis, or
//! > other financial activities may result in substantial losses, including total loss of capital. The Developers are not
//! > liable for any losses or damages arising from such use. Users assume full responsibility for validating the software's
//! > outputs and ensuring their suitability for intended purposes.
//! >
//! > This client may rely on third-party data or services (e.g., market feeds, APIs). The Developers do not control or verify
//! > the accuracy of these services and are not liable for any errors, delays, or losses resulting from their use. Users must
//! > comply with third-party terms and conditions.
//! >
//! > Users are solely responsible for ensuring compliance with all applicable financial, tax, and regulatory requirements in
//! > their jurisdiction. This includes obtaining necessary licenses or approvals for trading or investment activities. The
//! > Developers disclaim liability for any legal consequences arising from non-compliance.
//! >
//! > To the fullest extent permitted by law, the Developers shall not be liable for any direct, indirect, incidental,
//! > consequential, or punitive damages arising from the use or inability to use this software, including but not limited to
//! > loss of profits, data, or business opportunities.
//!
//! </details>
use crate::models;
use serde::{Deserialize, Serialize};

/// KeyboardButton : This object represents one button of the reply keyboard. At most one of the optional fields must be used to specify type of the button. For simple text buttons, *String* can be used instead of this object to specify the button text.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "request_users", skip_serializing_if = "Option::is_none")]
    pub request_users: Option<Box<models::KeyboardButtonRequestUsers>>,
    #[serde(rename = "request_chat", skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<Box<models::KeyboardButtonRequestChat>>,
    /// *Optional*. If *True*, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(rename = "request_contact", skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// *Optional*. If *True*, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(rename = "request_location", skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    #[serde(rename = "request_poll", skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<Box<models::KeyboardButtonPollType>>,
    #[serde(rename = "web_app", skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<models::WebAppInfo>>,
}

impl KeyboardButton {
    /// This object represents one button of the reply keyboard. At most one of the optional fields must be used to specify type of the button. For simple text buttons, *String* can be used instead of this object to specify the button text.
    pub fn new(text: String) -> KeyboardButton {
        KeyboardButton {
            text,
            request_users: None,
            request_chat: None,
            request_contact: None,
            request_location: None,
            request_poll: None,
            web_app: None,
        }
    }
}

