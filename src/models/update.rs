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

/// Update : This [object](https://core.telegram.org/bots/api/#available-types) represents an incoming update.   At most **one** of the optional parameters can be present in any given update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[serde(rename = "update_id")]
    pub update_id: i32,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::Message>>,
    #[serde(rename = "edited_message", skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<models::Message>>,
    #[serde(rename = "channel_post", skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<models::Message>>,
    #[serde(rename = "edited_channel_post", skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<models::Message>>,
    #[serde(rename = "business_connection", skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<Box<models::BusinessConnection>>,
    #[serde(rename = "business_message", skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Box<models::Message>>,
    #[serde(rename = "edited_business_message", skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Box<models::Message>>,
    #[serde(rename = "deleted_business_messages", skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<Box<models::BusinessMessagesDeleted>>,
    #[serde(rename = "message_reaction", skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<Box<models::MessageReactionUpdated>>,
    #[serde(rename = "message_reaction_count", skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<Box<models::MessageReactionCountUpdated>>,
    #[serde(rename = "inline_query", skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<Box<models::InlineQuery>>,
    #[serde(rename = "chosen_inline_result", skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<Box<models::ChosenInlineResult>>,
    #[serde(rename = "callback_query", skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<Box<models::CallbackQuery>>,
    #[serde(rename = "shipping_query", skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<Box<models::ShippingQuery>>,
    #[serde(rename = "pre_checkout_query", skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<Box<models::PreCheckoutQuery>>,
    #[serde(rename = "purchased_paid_media", skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<Box<models::PaidMediaPurchased>>,
    #[serde(rename = "poll", skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<models::Poll>>,
    #[serde(rename = "poll_answer", skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<Box<models::PollAnswer>>,
    #[serde(rename = "my_chat_member", skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<Box<models::ChatMemberUpdated>>,
    #[serde(rename = "chat_member", skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<Box<models::ChatMemberUpdated>>,
    #[serde(rename = "chat_join_request", skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<Box<models::ChatJoinRequest>>,
    #[serde(rename = "chat_boost", skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<Box<models::ChatBoostUpdated>>,
    #[serde(rename = "removed_chat_boost", skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<Box<models::ChatBoostRemoved>>,
}

impl Update {
    /// This [object](https://core.telegram.org/bots/api/#available-types) represents an incoming update.   At most **one** of the optional parameters can be present in any given update.
    pub fn new(update_id: i32) -> Update {
        Update {
            update_id,
            message: None,
            edited_message: None,
            channel_post: None,
            edited_channel_post: None,
            business_connection: None,
            business_message: None,
            edited_business_message: None,
            deleted_business_messages: None,
            message_reaction: None,
            message_reaction_count: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
            shipping_query: None,
            pre_checkout_query: None,
            purchased_paid_media: None,
            poll: None,
            poll_answer: None,
            my_chat_member: None,
            chat_member: None,
            chat_join_request: None,
            chat_boost: None,
            removed_chat_boost: None,
        }
    }
}

