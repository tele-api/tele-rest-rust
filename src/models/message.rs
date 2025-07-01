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

/// Message : This object represents a message.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    #[serde(rename = "message_id")]
    pub message_id: i32,
    /// *Optional*. Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id", skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<Box<models::User>>,
    #[serde(rename = "sender_chat", skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<models::Chat>>,
    /// *Optional*. If the sender of the message boosted the chat, the number of boosts added by the user
    #[serde(rename = "sender_boost_count", skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i32>,
    #[serde(rename = "sender_business_bot", skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<Box<models::User>>,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[serde(rename = "date")]
    pub date: i32,
    /// *Optional*. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[serde(rename = "business_connection_id", skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(rename = "chat")]
    pub chat: Box<models::Chat>,
    #[serde(rename = "forward_origin", skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<Box<models::MessageOrigin>>,
    /// *Optional*. *True*, if the message is sent to a forum topic
    #[serde(rename = "is_topic_message", skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    /// *Optional*. *True*, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(rename = "is_automatic_forward", skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    #[serde(rename = "reply_to_message", skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<models::Message>>,
    #[serde(rename = "external_reply", skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<Box<models::ExternalReplyInfo>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::TextQuote>>,
    #[serde(rename = "reply_to_story", skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Box<models::Story>>,
    #[serde(rename = "via_bot", skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<Box<models::User>>,
    /// *Optional*. Date the message was last edited in Unix time
    #[serde(rename = "edit_date", skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i32>,
    /// *Optional*. *True*, if the message can't be forwarded
    #[serde(rename = "has_protected_content", skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// *Optional*. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[serde(rename = "is_from_offline", skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,
    /// *Optional*. The unique identifier of a media message group this message belongs to
    #[serde(rename = "media_group_id", skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// *Optional*. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(rename = "author_signature", skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// *Optional*. The number of Telegram Stars that were paid by the sender of the message to send it
    #[serde(rename = "paid_star_count", skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<i32>,
    /// *Optional*. For text messages, the actual UTF-8 text of the message
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// *Optional*. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<models::MessageEntity>>,
    #[serde(rename = "link_preview_options", skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<models::LinkPreviewOptions>>,
    /// *Optional*. Unique identifier of the message effect added to the message
    #[serde(rename = "effect_id", skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,
    #[serde(rename = "animation", skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<models::Animation>>,
    #[serde(rename = "audio", skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<models::Audio>>,
    #[serde(rename = "document", skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<models::Document>>,
    #[serde(rename = "paid_media", skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Box<models::PaidMediaInfo>>,
    /// *Optional*. Message is a photo, available sizes of the photo
    #[serde(rename = "photo", skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<models::PhotoSize>>,
    #[serde(rename = "sticker", skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<models::Sticker>>,
    #[serde(rename = "story", skip_serializing_if = "Option::is_none")]
    pub story: Option<Box<models::Story>>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<models::Video>>,
    #[serde(rename = "video_note", skip_serializing_if = "Option::is_none")]
    pub video_note: Option<Box<models::VideoNote>>,
    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<Box<models::Voice>>,
    /// *Optional*. Caption for the animation, audio, document, paid media, photo, video or voice
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// *Optional*. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities", skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<models::MessageEntity>>,
    /// *Optional*. True, if the caption must be shown above the message media
    #[serde(rename = "show_caption_above_media", skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// *Optional*. *True*, if the message media is covered by a spoiler animation
    #[serde(rename = "has_media_spoiler", skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<models::Contact>>,
    #[serde(rename = "dice", skip_serializing_if = "Option::is_none")]
    pub dice: Option<Box<models::Dice>>,
    #[serde(rename = "game", skip_serializing_if = "Option::is_none")]
    pub game: Option<Box<models::Game>>,
    #[serde(rename = "poll", skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<models::Poll>>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<models::Venue>>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::Location>>,
    /// *Optional*. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(rename = "new_chat_members", skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<models::User>>,
    #[serde(rename = "left_chat_member", skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<Box<models::User>>,
    /// *Optional*. A chat title was changed to this value
    #[serde(rename = "new_chat_title", skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// *Optional*. A chat photo was change to this value
    #[serde(rename = "new_chat_photo", skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<models::PhotoSize>>,
    /// *Optional*. Service message: the chat photo was deleted
    #[serde(rename = "delete_chat_photo", skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    /// *Optional*. Service message: the group has been created
    #[serde(rename = "group_chat_created", skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    /// *Optional*. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\\_to\\_message if someone replies to a very first message in a directly created supergroup.
    #[serde(rename = "supergroup_chat_created", skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    /// *Optional*. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\\_to\\_message if someone replies to a very first message in a channel.
    #[serde(rename = "channel_chat_created", skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    #[serde(rename = "message_auto_delete_timer_changed", skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<Box<models::MessageAutoDeleteTimerChanged>>,
    /// *Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "migrate_to_chat_id", skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i32>,
    /// *Optional*. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "migrate_from_chat_id", skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i32>,
    #[serde(rename = "pinned_message", skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<models::MaybeInaccessibleMessage>>,
    #[serde(rename = "invoice", skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<models::Invoice>>,
    #[serde(rename = "successful_payment", skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<Box<models::SuccessfulPayment>>,
    #[serde(rename = "refunded_payment", skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<Box<models::RefundedPayment>>,
    #[serde(rename = "users_shared", skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<Box<models::UsersShared>>,
    #[serde(rename = "chat_shared", skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<Box<models::ChatShared>>,
    #[serde(rename = "gift", skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<models::GiftInfo>>,
    #[serde(rename = "unique_gift", skip_serializing_if = "Option::is_none")]
    pub unique_gift: Option<Box<models::UniqueGiftInfo>>,
    /// *Optional*. The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/widgets/login)
    #[serde(rename = "connected_website", skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    #[serde(rename = "write_access_allowed", skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<Box<models::WriteAccessAllowed>>,
    #[serde(rename = "passport_data", skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<Box<models::PassportData>>,
    #[serde(rename = "proximity_alert_triggered", skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<Box<models::ProximityAlertTriggered>>,
    #[serde(rename = "boost_added", skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<Box<models::ChatBoostAdded>>,
    #[serde(rename = "chat_background_set", skip_serializing_if = "Option::is_none")]
    pub chat_background_set: Option<Box<models::ChatBackground>>,
    #[serde(rename = "forum_topic_created", skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<Box<models::ForumTopicCreated>>,
    #[serde(rename = "forum_topic_edited", skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<Box<models::ForumTopicEdited>>,
    #[serde(rename = "forum_topic_closed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<Option<serde_json::Value>>,
    #[serde(rename = "forum_topic_reopened", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<Option<serde_json::Value>>,
    #[serde(rename = "general_forum_topic_hidden", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<Option<serde_json::Value>>,
    #[serde(rename = "general_forum_topic_unhidden", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<Option<serde_json::Value>>,
    #[serde(rename = "giveaway_created", skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<Box<models::GiveawayCreated>>,
    #[serde(rename = "giveaway", skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Box<models::Giveaway>>,
    #[serde(rename = "giveaway_winners", skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<Box<models::GiveawayWinners>>,
    #[serde(rename = "giveaway_completed", skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<models::GiveawayCompleted>>,
    #[serde(rename = "paid_message_price_changed", skip_serializing_if = "Option::is_none")]
    pub paid_message_price_changed: Option<Box<models::PaidMessagePriceChanged>>,
    #[serde(rename = "video_chat_scheduled", skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<Box<models::VideoChatScheduled>>,
    #[serde(rename = "video_chat_started", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<Option<serde_json::Value>>,
    #[serde(rename = "video_chat_ended", skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<Box<models::VideoChatEnded>>,
    #[serde(rename = "video_chat_participants_invited", skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<Box<models::VideoChatParticipantsInvited>>,
    #[serde(rename = "web_app_data", skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<Box<models::WebAppData>>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
}

impl Message {
    /// This object represents a message.
    pub fn new(message_id: i32, date: i32, chat: models::Chat) -> Message {
        Message {
            message_id,
            message_thread_id: None,
            from: None,
            sender_chat: None,
            sender_boost_count: None,
            sender_business_bot: None,
            date,
            business_connection_id: None,
            chat: Box::new(chat),
            forward_origin: None,
            is_topic_message: None,
            is_automatic_forward: None,
            reply_to_message: None,
            external_reply: None,
            quote: None,
            reply_to_story: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            is_from_offline: None,
            media_group_id: None,
            author_signature: None,
            paid_star_count: None,
            text: None,
            entities: None,
            link_preview_options: None,
            effect_id: None,
            animation: None,
            audio: None,
            document: None,
            paid_media: None,
            photo: None,
            sticker: None,
            story: None,
            video: None,
            video_note: None,
            voice: None,
            caption: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_media_spoiler: None,
            contact: None,
            dice: None,
            game: None,
            poll: None,
            venue: None,
            location: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: None,
            group_chat_created: None,
            supergroup_chat_created: None,
            channel_chat_created: None,
            message_auto_delete_timer_changed: None,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            invoice: None,
            successful_payment: None,
            refunded_payment: None,
            users_shared: None,
            chat_shared: None,
            gift: None,
            unique_gift: None,
            connected_website: None,
            write_access_allowed: None,
            passport_data: None,
            proximity_alert_triggered: None,
            boost_added: None,
            chat_background_set: None,
            forum_topic_created: None,
            forum_topic_edited: None,
            forum_topic_closed: None,
            forum_topic_reopened: None,
            general_forum_topic_hidden: None,
            general_forum_topic_unhidden: None,
            giveaway_created: None,
            giveaway: None,
            giveaway_winners: None,
            giveaway_completed: None,
            paid_message_price_changed: None,
            video_chat_scheduled: None,
            video_chat_started: None,
            video_chat_ended: None,
            video_chat_participants_invited: None,
            web_app_data: None,
            reply_markup: None,
        }
    }
}

