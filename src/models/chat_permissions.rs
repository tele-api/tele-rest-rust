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

/// ChatPermissions : Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    /// *Optional*. *True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[serde(rename = "can_send_messages", skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send audios
    #[serde(rename = "can_send_audios", skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send documents
    #[serde(rename = "can_send_documents", skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send photos
    #[serde(rename = "can_send_photos", skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send videos
    #[serde(rename = "can_send_videos", skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send video notes
    #[serde(rename = "can_send_video_notes", skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send voice notes
    #[serde(rename = "can_send_voice_notes", skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send polls
    #[serde(rename = "can_send_polls", skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(rename = "can_send_other_messages", skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to add web page previews to their messages
    #[serde(rename = "can_add_web_page_previews", skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(rename = "can_change_info", skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to invite new users to the chat
    #[serde(rename = "can_invite_users", skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(rename = "can_pin_messages", skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// *Optional*. *True*, if the user is allowed to create forum topics. If omitted defaults to the value of can\\_pin\\_messages
    #[serde(rename = "can_manage_topics", skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

impl ChatPermissions {
    /// Describes actions that a non-administrator user is allowed to take in a chat.
    pub fn new() -> ChatPermissions {
        ChatPermissions {
            can_send_messages: None,
            can_send_audios: None,
            can_send_documents: None,
            can_send_photos: None,
            can_send_videos: None,
            can_send_video_notes: None,
            can_send_voice_notes: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }
    }
}

