//! # Telegram Bot API - REST API Client
//! 
//! Auto-generated OpenAPI schema
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-01T14:14:23.986122366Z[Etc/UTC]
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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditMessageTextPostRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(rename = "business_connection_id", skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(rename = "chat_id", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Box<models::EditMessageTextPostRequestChatId>>,
    /// Required if *inline\\_message\\_id* is not specified. Identifier of the message to edit
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message
    #[serde(rename = "inline_message_id", skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing
    #[serde(rename = "text")]
    pub text: String,
    /// Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "parse_mode", skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\\_mode*
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<models::MessageEntity>>,
    #[serde(rename = "link_preview_options", skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<models::LinkPreviewOptions>>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
}

impl EditMessageTextPostRequest {
    pub fn new(text: String) -> EditMessageTextPostRequest {
        EditMessageTextPostRequest {
            business_connection_id: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            text,
            parse_mode: None,
            entities: None,
            link_preview_options: None,
            reply_markup: None,
        }
    }
}

