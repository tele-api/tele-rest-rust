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

/// InlineQueryResultContact : Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use *input\\_message\\_content* to send a message with the specified content instead of the contact.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be *contact*
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    #[serde(rename = "id")]
    pub id: String,
    /// Contact's phone number
    #[serde(rename = "phone_number")]
    pub phone_number: String,
    /// Contact's first name
    #[serde(rename = "first_name")]
    pub first_name: String,
    /// *Optional*. Contact's last name
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    #[serde(rename = "vcard", skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
    #[serde(rename = "input_message_content", skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<models::InputMessageContent>>,
    /// *Optional*. Url of the thumbnail for the result
    #[serde(rename = "thumbnail_url", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// *Optional*. Thumbnail width
    #[serde(rename = "thumbnail_width", skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,
    /// *Optional*. Thumbnail height
    #[serde(rename = "thumbnail_height", skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,
}

impl InlineQueryResultContact {
    /// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use *input\\_message\\_content* to send a message with the specified content instead of the contact.
    pub fn new(r#type: String, id: String, phone_number: String, first_name: String) -> InlineQueryResultContact {
        InlineQueryResultContact {
            r#type,
            id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }
}

