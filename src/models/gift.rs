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

/// Gift : This object represents a gift that can be sent by the bot.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gift {
    /// Unique identifier of the gift
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sticker")]
    pub sticker: Box<models::Sticker>,
    /// The number of Telegram Stars that must be paid to send the sticker
    #[serde(rename = "star_count")]
    pub star_count: i32,
    /// *Optional*. The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    #[serde(rename = "upgrade_star_count", skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<i32>,
    /// *Optional*. The total number of the gifts of this type that can be sent; for limited gifts only
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// *Optional*. The number of remaining gifts of this type that can be sent; for limited gifts only
    #[serde(rename = "remaining_count", skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i32>,
}

impl Gift {
    /// This object represents a gift that can be sent by the bot.
    pub fn new(id: String, sticker: models::Sticker, star_count: i32) -> Gift {
        Gift {
            id,
            sticker: Box::new(sticker),
            star_count,
            upgrade_star_count: None,
            total_count: None,
            remaining_count: None,
        }
    }
}

