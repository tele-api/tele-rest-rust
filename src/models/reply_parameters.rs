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

/// ReplyParameters : Describes reply parameters for the message that is being sent.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat *chat\\_id* if it is specified
    #[serde(rename = "message_id")]
    pub message_id: i32,
    #[serde(rename = "chat_id", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Box<models::ReplyParametersChatId>>,
    /// *Optional*. Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic. Always *True* for messages sent on behalf of a business account.
    #[serde(rename = "allow_sending_without_reply", skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// *Optional*. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\\_emoji* entities. The message will fail to send if the quote isn't found in the original message.
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// *Optional*. Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "quote_parse_mode", skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    /// *Optional*. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote\\_parse\\_mode*.
    #[serde(rename = "quote_entities", skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<models::MessageEntity>>,
    /// *Optional*. Position of the quote in the original message in UTF-16 code units
    #[serde(rename = "quote_position", skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i32>,
}

impl ReplyParameters {
    /// Describes reply parameters for the message that is being sent.
    pub fn new(message_id: i32) -> ReplyParameters {
        ReplyParameters {
            message_id,
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
        }
    }
}

