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

/// KeyboardButtonRequestChat : This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. [More about requesting chats »](https://core.telegram.org/bots/features#chat-and-user-selection).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the [ChatShared](https://core.telegram.org/bots/api/#chatshared) object. Must be unique within the message
    #[serde(rename = "request_id")]
    pub request_id: i32,
    /// Pass *True* to request a channel chat, pass *False* to request a group or a supergroup chat.
    #[serde(rename = "chat_is_channel")]
    pub chat_is_channel: bool,
    /// *Optional*. Pass *True* to request a forum supergroup, pass *False* to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[serde(rename = "chat_is_forum", skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// *Optional*. Pass *True* to request a supergroup or a channel with a username, pass *False* to request a chat without a username. If not specified, no additional restrictions are applied.
    #[serde(rename = "chat_has_username", skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// *Optional*. Pass *True* to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[serde(rename = "chat_is_created", skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    #[serde(rename = "user_administrator_rights", skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<Box<models::ChatAdministratorRights>>,
    #[serde(rename = "bot_administrator_rights", skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<Box<models::ChatAdministratorRights>>,
    /// *Optional*. Pass *True* to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[serde(rename = "bot_is_member", skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    /// *Optional*. Pass *True* to request the chat's title
    #[serde(rename = "request_title", skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    /// *Optional*. Pass *True* to request the chat's username
    #[serde(rename = "request_username", skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// *Optional*. Pass *True* to request the chat's photo
    #[serde(rename = "request_photo", skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

impl KeyboardButtonRequestChat {
    /// This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. [More about requesting chats »](https://core.telegram.org/bots/features#chat-and-user-selection).
    pub fn new(request_id: i32, chat_is_channel: bool) -> KeyboardButtonRequestChat {
        KeyboardButtonRequestChat {
            request_id,
            chat_is_channel,
            chat_is_forum: None,
            chat_has_username: None,
            chat_is_created: None,
            user_administrator_rights: None,
            bot_administrator_rights: None,
            bot_is_member: None,
            request_title: None,
            request_username: None,
            request_photo: None,
        }
    }
}

