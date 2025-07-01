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

/// ChatMemberUpdated : This object represents changes in the status of a chat member.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    #[serde(rename = "chat")]
    pub chat: Box<models::Chat>,
    #[serde(rename = "from")]
    pub from: Box<models::User>,
    /// Date the change was done in Unix time
    #[serde(rename = "date")]
    pub date: i32,
    #[serde(rename = "old_chat_member")]
    pub old_chat_member: Box<models::ChatMember>,
    #[serde(rename = "new_chat_member")]
    pub new_chat_member: Box<models::ChatMember>,
    #[serde(rename = "invite_link", skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<models::ChatInviteLink>>,
    /// *Optional*. True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[serde(rename = "via_join_request", skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    /// *Optional*. True, if the user joined the chat via a chat folder invite link
    #[serde(rename = "via_chat_folder_invite_link", skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

impl ChatMemberUpdated {
    /// This object represents changes in the status of a chat member.
    pub fn new(chat: models::Chat, from: models::User, date: i32, old_chat_member: models::ChatMember, new_chat_member: models::ChatMember) -> ChatMemberUpdated {
        ChatMemberUpdated {
            chat: Box::new(chat),
            from: Box::new(from),
            date,
            old_chat_member: Box::new(old_chat_member),
            new_chat_member: Box::new(new_chat_member),
            invite_link: None,
            via_join_request: None,
            via_chat_folder_invite_link: None,
        }
    }
}

