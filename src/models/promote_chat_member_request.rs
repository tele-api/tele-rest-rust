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

/// PromoteChatMemberRequest : Request parameters for promoteChatMember
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromoteChatMemberRequest {
    #[serde(rename = "chat_id")]
    pub chat_id: Box<models::SendMessageRequestChatId>,
    /// Unique identifier of the target user
    #[serde(rename = "user_id")]
    pub user_id: i32,
    /// Pass *True* if the administrator's presence in the chat is hidden
    #[serde(rename = "is_anonymous", skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.
    #[serde(rename = "can_manage_chat", skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Pass *True* if the administrator can delete messages of other users
    #[serde(rename = "can_delete_messages", skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Pass *True* if the administrator can manage video chats
    #[serde(rename = "can_manage_video_chats", skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    #[serde(rename = "can_restrict_members", skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(rename = "can_promote_members", skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Pass *True* if the administrator can change chat title, photo and other settings
    #[serde(rename = "can_change_info", skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Pass *True* if the administrator can invite new users to the chat
    #[serde(rename = "can_invite_users", skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Pass *True* if the administrator can post stories to the chat
    #[serde(rename = "can_post_stories", skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    /// Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[serde(rename = "can_edit_stories", skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    /// Pass *True* if the administrator can delete stories posted by other users
    #[serde(rename = "can_delete_stories", skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    /// Pass *True* if the administrator can post messages in the channel, or access channel statistics; for channels only
    #[serde(rename = "can_post_messages", skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(rename = "can_edit_messages", skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Pass *True* if the administrator can pin messages; for supergroups only
    #[serde(rename = "can_pin_messages", skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(rename = "can_manage_topics", skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

impl PromoteChatMemberRequest {
    /// Request parameters for promoteChatMember
    pub fn new(chat_id: models::SendMessageRequestChatId, user_id: i32) -> PromoteChatMemberRequest {
        PromoteChatMemberRequest {
            chat_id: Box::new(chat_id),
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_post_stories: None,
            can_edit_stories: None,
            can_delete_stories: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }
    }
}

