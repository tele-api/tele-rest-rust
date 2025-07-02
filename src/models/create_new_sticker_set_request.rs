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

/// CreateNewStickerSetRequest : Request parameters for createNewStickerSet
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNewStickerSetRequest {
    /// User identifier of created sticker set owner
    #[serde(rename = "user_id")]
    pub user_id: i32,
    /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `\"_by_<bot_username>\"`. `<bot_username>` is case insensitive. 1-64 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// Sticker set title, 1-64 characters
    #[serde(rename = "title")]
    pub title: String,
    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    #[serde(rename = "stickers")]
    pub stickers: Vec<models::InputSticker>,
    /// Type of stickers in the set, pass “regular”, “mask”, or “custom\\_emoji”. By default, a regular sticker set is created.
    #[serde(rename = "sticker_type", skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<StickerTypeEnum>,
    /// Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[serde(rename = "needs_repainting", skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

impl CreateNewStickerSetRequest {
    /// Request parameters for createNewStickerSet
    pub fn new(user_id: i32, name: String, title: String, stickers: Vec<models::InputSticker>) -> CreateNewStickerSetRequest {
        CreateNewStickerSetRequest {
            user_id,
            name,
            title,
            stickers,
            sticker_type: None,
            needs_repainting: None,
        }
    }
}
/// Type of stickers in the set, pass “regular”, “mask”, or “custom\\_emoji”. By default, a regular sticker set is created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StickerTypeEnum {
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

impl Default for StickerTypeEnum {
    fn default() -> StickerTypeEnum {
        Self::Mask
    }
}

