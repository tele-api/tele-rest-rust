//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-02T07:03:16.715318580Z[Etc/UTC]
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

/// ChatFullInfo : This object contains full information about a chat.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "id")]
    pub id: i32,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// *Optional*. Title, for supergroups, channels and group chats
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// *Optional*. Username, for private chats, supergroups and channels if available
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// *Optional*. First name of the other party in a private chat
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// *Optional*. Last name of the other party in a private chat
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// *Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    #[serde(rename = "is_forum", skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api/#accent-colors) for more details.
    #[serde(rename = "accent_color_id")]
    pub accent_color_id: i32,
    /// The maximum number of reactions that can be set on a message in the chat
    #[serde(rename = "max_reaction_count")]
    pub max_reaction_count: i32,
    #[serde(rename = "photo", skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<models::ChatPhoto>>,
    /// *Optional*. If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels
    #[serde(rename = "active_usernames", skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    #[serde(rename = "birthdate", skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Box<models::Birthdate>>,
    #[serde(rename = "business_intro", skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<Box<models::BusinessIntro>>,
    #[serde(rename = "business_location", skip_serializing_if = "Option::is_none")]
    pub business_location: Option<Box<models::BusinessLocation>>,
    #[serde(rename = "business_opening_hours", skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<Box<models::BusinessOpeningHours>>,
    #[serde(rename = "personal_chat", skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<models::Chat>>,
    /// *Optional*. List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api/#reactiontypeemoji) are allowed.
    #[serde(rename = "available_reactions", skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<models::ReactionType>>,
    /// *Optional*. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[serde(rename = "background_custom_emoji_id", skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// *Optional*. Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api/#profile-accent-colors) for more details.
    #[serde(rename = "profile_accent_color_id", skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i32>,
    /// *Optional*. Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[serde(rename = "profile_background_custom_emoji_id", skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    /// *Optional*. Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[serde(rename = "emoji_status_custom_emoji_id", skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// *Optional*. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[serde(rename = "emoji_status_expiration_date", skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i32>,
    /// *Optional*. Bio of the other party in a private chat
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// *Optional*. *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user
    #[serde(rename = "has_private_forwards", skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// *Optional*. *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[serde(rename = "has_restricted_voice_and_video_messages", skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// *Optional*. *True*, if users need to join the supergroup before they can send messages
    #[serde(rename = "join_to_send_messages", skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// *Optional*. *True*, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[serde(rename = "join_by_request", skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// *Optional*. Description, for groups, supergroups and channel chats
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// *Optional*. Primary invite link, for groups, supergroups and channel chats
    #[serde(rename = "invite_link", skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    #[serde(rename = "pinned_message", skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<models::Message>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::ChatPermissions>>,
    #[serde(rename = "accepted_gift_types")]
    pub accepted_gift_types: Box<models::AcceptedGiftTypes>,
    /// *Optional*. *True*, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[serde(rename = "can_send_paid_media", skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,
    /// *Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[serde(rename = "slow_mode_delay", skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i32>,
    /// *Optional*. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[serde(rename = "unrestrict_boost_count", skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i32>,
    /// *Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[serde(rename = "message_auto_delete_time", skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i32>,
    /// *Optional*. *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[serde(rename = "has_aggressive_anti_spam_enabled", skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// *Optional*. *True*, if non-administrators can only get the list of bots and administrators in the chat
    #[serde(rename = "has_hidden_members", skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// *Optional*. *True*, if messages from the chat can't be forwarded to other chats
    #[serde(rename = "has_protected_content", skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// *Optional*. *True*, if new chat members will have access to old messages; available only to chat administrators
    #[serde(rename = "has_visible_history", skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// *Optional*. For supergroups, name of the group sticker set
    #[serde(rename = "sticker_set_name", skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// *Optional*. *True*, if the bot can change the group sticker set
    #[serde(rename = "can_set_sticker_set", skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// *Optional*. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[serde(rename = "custom_emoji_sticker_set_name", skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    /// *Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "linked_chat_id", skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i32>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::ChatLocation>>,
}

impl ChatFullInfo {
    /// This object contains full information about a chat.
    pub fn new(id: i32, r#type: TypeEnum, accent_color_id: i32, max_reaction_count: i32, accepted_gift_types: models::AcceptedGiftTypes) -> ChatFullInfo {
        ChatFullInfo {
            id,
            r#type,
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            is_forum: None,
            accent_color_id,
            max_reaction_count,
            photo: None,
            active_usernames: None,
            birthdate: None,
            business_intro: None,
            business_location: None,
            business_opening_hours: None,
            personal_chat: None,
            available_reactions: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            join_to_send_messages: None,
            join_by_request: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            accepted_gift_types: Box::new(accepted_gift_types),
            can_send_paid_media: None,
            slow_mode_delay: None,
            unrestrict_boost_count: None,
            message_auto_delete_time: None,
            has_aggressive_anti_spam_enabled: None,
            has_hidden_members: None,
            has_protected_content: None,
            has_visible_history: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            custom_emoji_sticker_set_name: None,
            linked_chat_id: None,
            location: None,
        }
    }
}
/// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    Supergroup,
    #[serde(rename = "channel")]
    Channel,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Private
    }
}

