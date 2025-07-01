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

/// InputProfilePhoto : This object describes a profile photo to set. Currently, it can be one of  * [InputProfilePhotoStatic](https://core.telegram.org/bots/api/#inputprofilephotostatic) * [InputProfilePhotoAnimated](https://core.telegram.org/bots/api/#inputprofilephotoanimated)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputProfilePhoto {
    /// Type of the profile photo, must be *animated*
    #[serde(rename = "type")]
    pub r#type: String,
    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://\\<file\\_attach\\_name\\>” if the photo was uploaded using multipart/form-data under \\<file\\_attach\\_name\\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[serde(rename = "photo")]
    pub photo: String,
    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://\\<file\\_attach\\_name\\>” if the photo was uploaded using multipart/form-data under \\<file\\_attach\\_name\\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[serde(rename = "animation")]
    pub animation: String,
    /// *Optional*. Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[serde(rename = "main_frame_timestamp", skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,
}

impl InputProfilePhoto {
    /// This object describes a profile photo to set. Currently, it can be one of  * [InputProfilePhotoStatic](https://core.telegram.org/bots/api/#inputprofilephotostatic) * [InputProfilePhotoAnimated](https://core.telegram.org/bots/api/#inputprofilephotoanimated)
    pub fn new(r#type: String, photo: String, animation: String) -> InputProfilePhoto {
        InputProfilePhoto {
            r#type,
            photo,
            animation,
            main_frame_timestamp: None,
        }
    }
}

