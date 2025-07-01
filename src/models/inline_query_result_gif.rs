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

/// InlineQueryResultGif : Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use *input\\_message\\_content* to send a message with the specified content instead of the animation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be *gif*
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    #[serde(rename = "id")]
    pub id: String,
    /// A valid URL for the GIF file
    #[serde(rename = "gif_url")]
    pub gif_url: String,
    /// *Optional*. Width of the GIF
    #[serde(rename = "gif_width", skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i32>,
    /// *Optional*. Height of the GIF
    #[serde(rename = "gif_height", skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i32>,
    /// *Optional*. Duration of the GIF in seconds
    #[serde(rename = "gif_duration", skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i32>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    /// *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(rename = "thumbnail_mime_type", skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<ThumbnailMimeTypeEnum>,
    /// *Optional*. Title for the result
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// *Optional*. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "parse_mode", skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode*
    #[serde(rename = "caption_entities", skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<models::MessageEntity>>,
    /// *Optional*. Pass *True*, if the caption must be shown above the message media
    #[serde(rename = "show_caption_above_media", skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
    #[serde(rename = "input_message_content", skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<models::InputMessageContent>>,
}

impl InlineQueryResultGif {
    /// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use *input\\_message\\_content* to send a message with the specified content instead of the animation.
    pub fn new(r#type: String, id: String, gif_url: String, thumbnail_url: String) -> InlineQueryResultGif {
        InlineQueryResultGif {
            r#type,
            id,
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumbnail_url,
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
/// *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ThumbnailMimeTypeEnum {
    #[serde(rename = "image/jpeg")]
    ImageSlashJpeg,
    #[serde(rename = "image/gif")]
    ImageSlashGif,
    #[serde(rename = "video/mp4")]
    VideoSlashMp4,
}

impl Default for ThumbnailMimeTypeEnum {
    fn default() -> ThumbnailMimeTypeEnum {
        Self::ImageSlashJpeg
    }
}

