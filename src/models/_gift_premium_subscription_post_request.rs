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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiftPremiumSubscriptionPostRequest {
    /// Unique identifier of the target user who will receive a Telegram Premium subscription
    #[serde(rename = "user_id")]
    pub user_id: i32,
    /// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    #[serde(rename = "month_count")]
    pub month_count: MonthCountEnum,
    /// Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    #[serde(rename = "star_count")]
    pub star_count: i32,
    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\\_emoji” are ignored.
    #[serde(rename = "text_parse_mode", skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\\_parse\\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\\_emoji” are ignored.
    #[serde(rename = "text_entities", skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<models::MessageEntity>>,
}

impl GiftPremiumSubscriptionPostRequest {
    pub fn new(user_id: i32, month_count: MonthCountEnum, star_count: i32) -> GiftPremiumSubscriptionPostRequest {
        GiftPremiumSubscriptionPostRequest {
            user_id,
            month_count,
            star_count,
            text: None,
            text_parse_mode: None,
            text_entities: None,
        }
    }
}
/// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonthCountEnum {
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "6")]
    Variant6,
    #[serde(rename = "12")]
    Variant12,
}

impl Default for MonthCountEnum {
    fn default() -> MonthCountEnum {
        Self::Variant3
    }
}

