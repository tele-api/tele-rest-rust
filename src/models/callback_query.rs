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

/// CallbackQuery : This object represents an incoming callback query from a callback button in an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\\_message\\_id* will be present. Exactly one of the fields *data* or *game\\_short\\_name* will be present.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "from")]
    pub from: Box<models::User>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::MaybeInaccessibleMessage>>,
    /// *Optional*. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(rename = "inline_message_id", skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api/#games).
    #[serde(rename = "chat_instance")]
    pub chat_instance: String,
    /// *Optional*. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// *Optional*. Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game
    #[serde(rename = "game_short_name", skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    /// This object represents an incoming callback query from a callback button in an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\\_message\\_id* will be present. Exactly one of the fields *data* or *game\\_short\\_name* will be present.
    pub fn new(id: String, from: models::User, chat_instance: String) -> CallbackQuery {
        CallbackQuery {
            id,
            from: Box::new(from),
            message: None,
            inline_message_id: None,
            chat_instance,
            data: None,
            game_short_name: None,
        }
    }
}

