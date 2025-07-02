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

/// OwnedGiftUnique : Describes a unique gift received and owned by a user or a chat.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OwnedGiftUnique {
    /// Type of the gift, always “unique”
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "gift")]
    pub gift: Box<models::UniqueGift>,
    /// *Optional*. Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[serde(rename = "owned_gift_id", skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    #[serde(rename = "sender_user", skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<Box<models::User>>,
    /// Date the gift was sent in Unix time
    #[serde(rename = "send_date")]
    pub send_date: i32,
    /// *Optional*. True, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(rename = "is_saved", skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// *Optional*. True, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[serde(rename = "can_be_transferred", skip_serializing_if = "Option::is_none")]
    pub can_be_transferred: Option<bool>,
    /// *Optional*. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(rename = "transfer_star_count", skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i32>,
}

impl OwnedGiftUnique {
    /// Describes a unique gift received and owned by a user or a chat.
    pub fn new(r#type: String, gift: models::UniqueGift, send_date: i32) -> OwnedGiftUnique {
        OwnedGiftUnique {
            r#type,
            gift: Box::new(gift),
            owned_gift_id: None,
            sender_user: None,
            send_date,
            is_saved: None,
            can_be_transferred: None,
            transfer_star_count: None,
        }
    }
}

