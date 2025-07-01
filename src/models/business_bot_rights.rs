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

/// BusinessBotRights : Represents the rights of a business bot.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessBotRights {
    /// *Optional*. True, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    #[serde(rename = "can_reply", skip_serializing_if = "Option::is_none")]
    pub can_reply: Option<bool>,
    /// *Optional*. True, if the bot can mark incoming private messages as read
    #[serde(rename = "can_read_messages", skip_serializing_if = "Option::is_none")]
    pub can_read_messages: Option<bool>,
    /// *Optional*. True, if the bot can delete messages sent by the bot
    #[serde(rename = "can_delete_sent_messages", skip_serializing_if = "Option::is_none")]
    pub can_delete_sent_messages: Option<bool>,
    /// *Optional*. True, if the bot can delete all private messages in managed chats
    #[serde(rename = "can_delete_all_messages", skip_serializing_if = "Option::is_none")]
    pub can_delete_all_messages: Option<bool>,
    /// *Optional*. True, if the bot can edit the first and last name of the business account
    #[serde(rename = "can_edit_name", skip_serializing_if = "Option::is_none")]
    pub can_edit_name: Option<bool>,
    /// *Optional*. True, if the bot can edit the bio of the business account
    #[serde(rename = "can_edit_bio", skip_serializing_if = "Option::is_none")]
    pub can_edit_bio: Option<bool>,
    /// *Optional*. True, if the bot can edit the profile photo of the business account
    #[serde(rename = "can_edit_profile_photo", skip_serializing_if = "Option::is_none")]
    pub can_edit_profile_photo: Option<bool>,
    /// *Optional*. True, if the bot can edit the username of the business account
    #[serde(rename = "can_edit_username", skip_serializing_if = "Option::is_none")]
    pub can_edit_username: Option<bool>,
    /// *Optional*. True, if the bot can change the privacy settings pertaining to gifts for the business account
    #[serde(rename = "can_change_gift_settings", skip_serializing_if = "Option::is_none")]
    pub can_change_gift_settings: Option<bool>,
    /// *Optional*. True, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    #[serde(rename = "can_view_gifts_and_stars", skip_serializing_if = "Option::is_none")]
    pub can_view_gifts_and_stars: Option<bool>,
    /// *Optional*. True, if the bot can convert regular gifts owned by the business account to Telegram Stars
    #[serde(rename = "can_convert_gifts_to_stars", skip_serializing_if = "Option::is_none")]
    pub can_convert_gifts_to_stars: Option<bool>,
    /// *Optional*. True, if the bot can transfer and upgrade gifts owned by the business account
    #[serde(rename = "can_transfer_and_upgrade_gifts", skip_serializing_if = "Option::is_none")]
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    /// *Optional*. True, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    #[serde(rename = "can_transfer_stars", skip_serializing_if = "Option::is_none")]
    pub can_transfer_stars: Option<bool>,
    /// *Optional*. True, if the bot can post, edit and delete stories on behalf of the business account
    #[serde(rename = "can_manage_stories", skip_serializing_if = "Option::is_none")]
    pub can_manage_stories: Option<bool>,
}

impl BusinessBotRights {
    /// Represents the rights of a business bot.
    pub fn new() -> BusinessBotRights {
        BusinessBotRights {
            can_reply: None,
            can_read_messages: None,
            can_delete_sent_messages: None,
            can_delete_all_messages: None,
            can_edit_name: None,
            can_edit_bio: None,
            can_edit_profile_photo: None,
            can_edit_username: None,
            can_change_gift_settings: None,
            can_view_gifts_and_stars: None,
            can_convert_gifts_to_stars: None,
            can_transfer_and_upgrade_gifts: None,
            can_transfer_stars: None,
            can_manage_stories: None,
        }
    }
}

