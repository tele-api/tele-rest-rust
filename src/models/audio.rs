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

/// Audio : This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[serde(rename = "file_unique_id")]
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by the sender
    #[serde(rename = "duration")]
    pub duration: i32,
    /// *Optional*. Performer of the audio as defined by the sender or by audio tags
    #[serde(rename = "performer", skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// *Optional*. Title of the audio as defined by the sender or by audio tags
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// *Optional*. Original filename as defined by the sender
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// *Optional*. MIME type of the file as defined by the sender
    #[serde(rename = "mime_type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(rename = "file_size", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<models::PhotoSize>>,
}

impl Audio {
    /// This object represents an audio file to be treated as music by the Telegram clients.
    pub fn new(file_id: String, file_unique_id: String, duration: i32) -> Audio {
        Audio {
            file_id,
            file_unique_id,
            duration,
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumbnail: None,
        }
    }
}

