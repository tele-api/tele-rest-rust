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

/// ExternalReplyInfo : This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    #[serde(rename = "origin")]
    pub origin: Box<models::MessageOrigin>,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<models::Chat>>,
    /// *Optional*. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(rename = "link_preview_options", skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<models::LinkPreviewOptions>>,
    #[serde(rename = "animation", skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<models::Animation>>,
    #[serde(rename = "audio", skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<models::Audio>>,
    #[serde(rename = "document", skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<models::Document>>,
    #[serde(rename = "paid_media", skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Box<models::PaidMediaInfo>>,
    /// *Optional*. Message is a photo, available sizes of the photo
    #[serde(rename = "photo", skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<models::PhotoSize>>,
    #[serde(rename = "sticker", skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<models::Sticker>>,
    #[serde(rename = "story", skip_serializing_if = "Option::is_none")]
    pub story: Option<Box<models::Story>>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<models::Video>>,
    #[serde(rename = "video_note", skip_serializing_if = "Option::is_none")]
    pub video_note: Option<Box<models::VideoNote>>,
    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<Box<models::Voice>>,
    /// *Optional*. *True*, if the message media is covered by a spoiler animation
    #[serde(rename = "has_media_spoiler", skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<models::Contact>>,
    #[serde(rename = "dice", skip_serializing_if = "Option::is_none")]
    pub dice: Option<Box<models::Dice>>,
    #[serde(rename = "game", skip_serializing_if = "Option::is_none")]
    pub game: Option<Box<models::Game>>,
    #[serde(rename = "giveaway", skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Box<models::Giveaway>>,
    #[serde(rename = "giveaway_winners", skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<Box<models::GiveawayWinners>>,
    #[serde(rename = "invoice", skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<models::Invoice>>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::Location>>,
    #[serde(rename = "poll", skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<models::Poll>>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<models::Venue>>,
}

impl ExternalReplyInfo {
    /// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
    pub fn new(origin: models::MessageOrigin) -> ExternalReplyInfo {
        ExternalReplyInfo {
            origin: Box::new(origin),
            chat: None,
            message_id: None,
            link_preview_options: None,
            animation: None,
            audio: None,
            document: None,
            paid_media: None,
            photo: None,
            sticker: None,
            story: None,
            video: None,
            video_note: None,
            voice: None,
            has_media_spoiler: None,
            contact: None,
            dice: None,
            game: None,
            giveaway: None,
            giveaway_winners: None,
            invoice: None,
            location: None,
            poll: None,
            venue: None,
        }
    }
}

