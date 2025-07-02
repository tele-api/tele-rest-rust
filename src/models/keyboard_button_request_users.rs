//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-02T07:03:16.715318580Z[Etc/UTC]
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

/// KeyboardButtonRequestUsers : This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. [More about requesting users »](https://core.telegram.org/bots/features#chat-and-user-selection)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request that will be received back in the [UsersShared](https://core.telegram.org/bots/api/#usersshared) object. Must be unique within the message
    #[serde(rename = "request_id")]
    pub request_id: i32,
    /// *Optional*. Pass *True* to request bots, pass *False* to request regular users. If not specified, no additional restrictions are applied.
    #[serde(rename = "user_is_bot", skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// *Optional*. Pass *True* to request premium users, pass *False* to request non-premium users. If not specified, no additional restrictions are applied.
    #[serde(rename = "user_is_premium", skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    /// *Optional*. The maximum number of users to be selected; 1-10. Defaults to 1.
    #[serde(rename = "max_quantity", skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i32>,
    /// *Optional*. Pass *True* to request the users' first and last names
    #[serde(rename = "request_name", skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    /// *Optional*. Pass *True* to request the users' usernames
    #[serde(rename = "request_username", skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// *Optional*. Pass *True* to request the users' photos
    #[serde(rename = "request_photo", skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

impl KeyboardButtonRequestUsers {
    /// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. [More about requesting users »](https://core.telegram.org/bots/features#chat-and-user-selection)
    pub fn new(request_id: i32) -> KeyboardButtonRequestUsers {
        KeyboardButtonRequestUsers {
            request_id,
            user_is_bot: None,
            user_is_premium: None,
            max_quantity: None,
            request_name: None,
            request_username: None,
            request_photo: None,
        }
    }
}

