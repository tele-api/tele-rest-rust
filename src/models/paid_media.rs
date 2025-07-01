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

/// PaidMedia : This object describes paid media. Currently, it can be one of  * [PaidMediaPreview](https://core.telegram.org/bots/api/#paidmediapreview) * [PaidMediaPhoto](https://core.telegram.org/bots/api/#paidmediaphoto) * [PaidMediaVideo](https://core.telegram.org/bots/api/#paidmediavideo)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaidMedia {
    /// Type of the paid media, always “video”
    #[serde(rename = "type")]
    pub r#type: String,
    /// *Optional*. Media width as defined by the sender
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// *Optional*. Media height as defined by the sender
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// *Optional*. Duration of the media in seconds as defined by the sender
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// The photo
    #[serde(rename = "photo")]
    pub photo: Vec<models::PhotoSize>,
    #[serde(rename = "video")]
    pub video: Box<models::Video>,
}

impl PaidMedia {
    /// This object describes paid media. Currently, it can be one of  * [PaidMediaPreview](https://core.telegram.org/bots/api/#paidmediapreview) * [PaidMediaPhoto](https://core.telegram.org/bots/api/#paidmediaphoto) * [PaidMediaVideo](https://core.telegram.org/bots/api/#paidmediavideo)
    pub fn new(r#type: String, photo: Vec<models::PhotoSize>, video: models::Video) -> PaidMedia {
        PaidMedia {
            r#type,
            width: None,
            height: None,
            duration: None,
            photo,
            video: Box::new(video),
        }
    }
}

