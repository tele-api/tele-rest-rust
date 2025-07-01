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

/// ChatMember : This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:  * [ChatMemberOwner](https://core.telegram.org/bots/api/#chatmemberowner) * [ChatMemberAdministrator](https://core.telegram.org/bots/api/#chatmemberadministrator) * [ChatMemberMember](https://core.telegram.org/bots/api/#chatmembermember) * [ChatMemberRestricted](https://core.telegram.org/bots/api/#chatmemberrestricted) * [ChatMemberLeft](https://core.telegram.org/bots/api/#chatmemberleft) * [ChatMemberBanned](https://core.telegram.org/bots/api/#chatmemberbanned)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMember {
    /// The member's status in the chat, always “kicked”
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "user")]
    pub user: Box<models::User>,
    /// *True*, if the user's presence in the chat is hidden
    #[serde(rename = "is_anonymous")]
    pub is_anonymous: bool,
    /// *Optional*. Custom title for this user
    #[serde(rename = "custom_title", skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
    /// *True*, if the bot is allowed to edit administrator privileges of that user
    #[serde(rename = "can_be_edited")]
    pub can_be_edited: bool,
    /// *True*, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.
    #[serde(rename = "can_manage_chat")]
    pub can_manage_chat: bool,
    /// *True*, if the administrator can delete messages of other users
    #[serde(rename = "can_delete_messages")]
    pub can_delete_messages: bool,
    /// *True*, if the administrator can manage video chats
    #[serde(rename = "can_manage_video_chats")]
    pub can_manage_video_chats: bool,
    /// *True*, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    #[serde(rename = "can_restrict_members")]
    pub can_restrict_members: bool,
    /// *True*, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    #[serde(rename = "can_promote_members")]
    pub can_promote_members: bool,
    /// *True*, if the user is allowed to change the chat title, photo and other settings
    #[serde(rename = "can_change_info")]
    pub can_change_info: bool,
    /// *True*, if the user is allowed to invite new users to the chat
    #[serde(rename = "can_invite_users")]
    pub can_invite_users: bool,
    /// *True*, if the administrator can post stories to the chat
    #[serde(rename = "can_post_stories")]
    pub can_post_stories: bool,
    /// *True*, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[serde(rename = "can_edit_stories")]
    pub can_edit_stories: bool,
    /// *True*, if the administrator can delete stories posted by other users
    #[serde(rename = "can_delete_stories")]
    pub can_delete_stories: bool,
    /// *Optional*. *True*, if the administrator can post messages in the channel, or access channel statistics; for channels only
    #[serde(rename = "can_post_messages", skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// *Optional*. *True*, if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(rename = "can_edit_messages", skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// *True*, if the user is allowed to pin messages
    #[serde(rename = "can_pin_messages")]
    pub can_pin_messages: bool,
    /// *True*, if the user is allowed to create forum topics
    #[serde(rename = "can_manage_topics")]
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    #[serde(rename = "until_date")]
    pub until_date: i32,
    /// *True*, if the user is a member of the chat at the moment of the request
    #[serde(rename = "is_member")]
    pub is_member: bool,
    /// *True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[serde(rename = "can_send_messages")]
    pub can_send_messages: bool,
    /// *True*, if the user is allowed to send audios
    #[serde(rename = "can_send_audios")]
    pub can_send_audios: bool,
    /// *True*, if the user is allowed to send documents
    #[serde(rename = "can_send_documents")]
    pub can_send_documents: bool,
    /// *True*, if the user is allowed to send photos
    #[serde(rename = "can_send_photos")]
    pub can_send_photos: bool,
    /// *True*, if the user is allowed to send videos
    #[serde(rename = "can_send_videos")]
    pub can_send_videos: bool,
    /// *True*, if the user is allowed to send video notes
    #[serde(rename = "can_send_video_notes")]
    pub can_send_video_notes: bool,
    /// *True*, if the user is allowed to send voice notes
    #[serde(rename = "can_send_voice_notes")]
    pub can_send_voice_notes: bool,
    /// *True*, if the user is allowed to send polls
    #[serde(rename = "can_send_polls")]
    pub can_send_polls: bool,
    /// *True*, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(rename = "can_send_other_messages")]
    pub can_send_other_messages: bool,
    /// *True*, if the user is allowed to add web page previews to their messages
    #[serde(rename = "can_add_web_page_previews")]
    pub can_add_web_page_previews: bool,
}

impl ChatMember {
    /// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:  * [ChatMemberOwner](https://core.telegram.org/bots/api/#chatmemberowner) * [ChatMemberAdministrator](https://core.telegram.org/bots/api/#chatmemberadministrator) * [ChatMemberMember](https://core.telegram.org/bots/api/#chatmembermember) * [ChatMemberRestricted](https://core.telegram.org/bots/api/#chatmemberrestricted) * [ChatMemberLeft](https://core.telegram.org/bots/api/#chatmemberleft) * [ChatMemberBanned](https://core.telegram.org/bots/api/#chatmemberbanned)
    pub fn new(status: String, user: models::User, is_anonymous: bool, can_be_edited: bool, can_manage_chat: bool, can_delete_messages: bool, can_manage_video_chats: bool, can_restrict_members: bool, can_promote_members: bool, can_change_info: bool, can_invite_users: bool, can_post_stories: bool, can_edit_stories: bool, can_delete_stories: bool, can_pin_messages: bool, can_manage_topics: bool, until_date: i32, is_member: bool, can_send_messages: bool, can_send_audios: bool, can_send_documents: bool, can_send_photos: bool, can_send_videos: bool, can_send_video_notes: bool, can_send_voice_notes: bool, can_send_polls: bool, can_send_other_messages: bool, can_add_web_page_previews: bool) -> ChatMember {
        ChatMember {
            status,
            user: Box::new(user),
            is_anonymous,
            custom_title: None,
            can_be_edited,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_stories,
            can_edit_stories,
            can_delete_stories,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages,
            can_manage_topics,
            until_date,
            is_member,
            can_send_messages,
            can_send_audios,
            can_send_documents,
            can_send_photos,
            can_send_videos,
            can_send_video_notes,
            can_send_voice_notes,
            can_send_polls,
            can_send_other_messages,
            can_add_web_page_previews,
        }
    }
}

