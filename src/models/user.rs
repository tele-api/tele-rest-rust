//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-02T09:17:04.370667370Z[Etc/UTC]
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

/// User : This object represents a Telegram user or bot.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "id")]
    pub id: i32,
    /// *True*, if this user is a bot
    #[serde(rename = "is_bot")]
    pub is_bot: bool,
    /// User's or bot's first name
    #[serde(rename = "first_name")]
    pub first_name: String,
    /// *Optional*. User's or bot's last name
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// *Optional*. User's or bot's username
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// *Optional*. [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    #[serde(rename = "language_code", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// *Optional*. *True*, if this user is a Telegram Premium user
    #[serde(rename = "is_premium", skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// *Optional*. *True*, if this user added the bot to the attachment menu
    #[serde(rename = "added_to_attachment_menu", skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// *Optional*. *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "can_join_groups", skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// *Optional*. *True*, if [privacy mode](https://core.telegram.org/bots/features#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "can_read_all_group_messages", skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// *Optional*. *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "supports_inline_queries", skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    /// *Optional*. *True*, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "can_connect_to_business", skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,
    /// *Optional*. *True*, if the bot has a main Web App. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "has_main_web_app", skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
}

impl User {
    /// This object represents a Telegram user or bot.
    pub fn new(id: i32, is_bot: bool, first_name: String) -> User {
        User {
            id,
            is_bot,
            first_name,
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
            can_connect_to_business: None,
            has_main_web_app: None,
        }
    }
}

