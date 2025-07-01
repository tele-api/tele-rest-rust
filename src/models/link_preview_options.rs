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

/// LinkPreviewOptions : Describes the options used for link preview generation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    /// *Optional*. *True*, if the link preview is disabled
    #[serde(rename = "is_disabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// *Optional*. URL to use for the link preview. If empty, then the first URL found in the message text will be used
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// *Optional*. *True*, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(rename = "prefer_small_media", skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    /// *Optional*. *True*, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(rename = "prefer_large_media", skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    /// *Optional*. *True*, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[serde(rename = "show_above_text", skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

impl LinkPreviewOptions {
    /// Describes the options used for link preview generation.
    pub fn new() -> LinkPreviewOptions {
        LinkPreviewOptions {
            is_disabled: None,
            url: None,
            prefer_small_media: None,
            prefer_large_media: None,
            show_above_text: None,
        }
    }
}

