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

/// Sticker : This object represents a sticker.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[serde(rename = "file_unique_id")]
    pub file_unique_id: String,
    /// Type of the sticker, currently one of “regular”, “mask”, “custom\\_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is\\_animated* and *is\\_video*.
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// Sticker width
    #[serde(rename = "width")]
    pub width: i32,
    /// Sticker height
    #[serde(rename = "height")]
    pub height: i32,
    /// *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
    #[serde(rename = "is_animated")]
    pub is_animated: bool,
    /// *True*, if the sticker is a [video sticker](https://telegram.org/blog/video-stickers-better-reactions)
    #[serde(rename = "is_video")]
    pub is_video: bool,
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<models::PhotoSize>>,
    /// *Optional*. Emoji associated with the sticker
    #[serde(rename = "emoji", skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// *Optional*. Name of the sticker set to which the sticker belongs
    #[serde(rename = "set_name", skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    #[serde(rename = "premium_animation", skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<Box<models::File>>,
    #[serde(rename = "mask_position", skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<Box<models::MaskPosition>>,
    /// *Optional*. For custom emoji stickers, unique identifier of the custom emoji
    #[serde(rename = "custom_emoji_id", skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// *Optional*. *True*, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[serde(rename = "needs_repainting", skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    /// *Optional*. File size in bytes
    #[serde(rename = "file_size", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
}

impl Sticker {
    /// This object represents a sticker.
    pub fn new(file_id: String, file_unique_id: String, r#type: TypeEnum, width: i32, height: i32, is_animated: bool, is_video: bool) -> Sticker {
        Sticker {
            file_id,
            file_unique_id,
            r#type,
            width,
            height,
            is_animated,
            is_video,
            thumbnail: None,
            emoji: None,
            set_name: None,
            premium_animation: None,
            mask_position: None,
            custom_emoji_id: None,
            needs_repainting: None,
            file_size: None,
        }
    }
}
/// Type of the sticker, currently one of “regular”, “mask”, “custom\\_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is\\_animated* and *is\\_video*.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Regular
    }
}

