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

/// SendVideoRequest : Request parameters for sendVideo
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendVideoRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(rename = "business_connection_id", skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(rename = "chat_id")]
    pub chat_id: Box<models::SendMessageRequestChatId>,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(rename = "message_thread_id", skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(rename = "video", deserialize_with = "Option::deserialize")]
    pub video: Option<String>,
    /// Duration of sent video in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Video width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Video height
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "thumbnail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Option<String>>,
    #[serde(rename = "cover", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cover: Option<Option<String>>,
    /// Start timestamp for the video in the message
    #[serde(rename = "start_timestamp", skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i32>,
    /// Video caption (may also be used when resending videos by *file\\_id*), 0-1024 characters after entities parsing
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "parse_mode", skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\\_mode*
    #[serde(rename = "caption_entities", skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<models::MessageEntity>>,
    /// Pass *True*, if the caption must be shown above the message media
    #[serde(rename = "show_caption_above_media", skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Pass *True* if the video needs to be covered with a spoiler animation
    #[serde(rename = "has_spoiler", skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Pass *True* if the uploaded video is suitable for streaming
    #[serde(rename = "supports_streaming", skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(rename = "disable_notification", skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(rename = "protect_content", skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(rename = "allow_paid_broadcast", skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(rename = "message_effect_id", skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(rename = "reply_parameters", skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<models::ReplyParameters>>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::SendMessageRequestReplyMarkup>>,
}

impl SendVideoRequest {
    /// Request parameters for sendVideo
    pub fn new(chat_id: models::SendMessageRequestChatId, video: Option<String>) -> SendVideoRequest {
        SendVideoRequest {
            business_connection_id: None,
            chat_id: Box::new(chat_id),
            message_thread_id: None,
            video,
            duration: None,
            width: None,
            height: None,
            thumbnail: None,
            cover: None,
            start_timestamp: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
            supports_streaming: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }
}

