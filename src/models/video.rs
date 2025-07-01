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

/// Video : This object represents a video file.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[serde(rename = "file_unique_id")]
    pub file_unique_id: String,
    /// Video width as defined by the sender
    #[serde(rename = "width")]
    pub width: i32,
    /// Video height as defined by the sender
    #[serde(rename = "height")]
    pub height: i32,
    /// Duration of the video in seconds as defined by the sender
    #[serde(rename = "duration")]
    pub duration: i32,
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<models::PhotoSize>>,
    /// *Optional*. Available sizes of the cover of the video in the message
    #[serde(rename = "cover", skip_serializing_if = "Option::is_none")]
    pub cover: Option<Vec<models::PhotoSize>>,
    /// *Optional*. Timestamp in seconds from which the video will play in the message
    #[serde(rename = "start_timestamp", skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i32>,
    /// *Optional*. Original filename as defined by the sender
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// *Optional*. MIME type of the file as defined by the sender
    #[serde(rename = "mime_type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(rename = "file_size", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
}

impl Video {
    /// This object represents a video file.
    pub fn new(file_id: String, file_unique_id: String, width: i32, height: i32, duration: i32) -> Video {
        Video {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumbnail: None,
            cover: None,
            start_timestamp: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

