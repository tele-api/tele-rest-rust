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
pub struct SendPollPostRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(rename = "business_connection_id", skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(rename = "chat_id")]
    pub chat_id: Box<models::SendMessagePostRequestChatId>,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(rename = "message_thread_id", skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// Poll question, 1-300 characters
    #[serde(rename = "question")]
    pub question: String,
    /// Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed
    #[serde(rename = "question_parse_mode", skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question\\_parse\\_mode*
    #[serde(rename = "question_entities", skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<models::MessageEntity>>,
    /// A JSON-serialized list of 2-10 answer options
    #[serde(rename = "options")]
    pub options: Vec<models::InputPollOption>,
    /// *True*, if the poll needs to be anonymous, defaults to *True*
    #[serde(rename = "is_anonymous", skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TypeEnum>,
    /// *True*, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False*
    #[serde(rename = "allows_multiple_answers", skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(rename = "correct_option_id", skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i32>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(rename = "explanation", skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "explanation_parse_mode", skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation\\_parse\\_mode*
    #[serde(rename = "explanation_entities", skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<models::MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\\_date*.
    #[serde(rename = "open_period", skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i32>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\\_period*.
    #[serde(rename = "close_date", skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i32>,
    /// Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(rename = "is_closed", skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(rename = "disable_notification", skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(rename = "protect_content", skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(rename = "allow_paid_broadcast", skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(rename = "message_effect_id", skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(rename = "reply_parameters", skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<Box<models::ReplyParameters>>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::SendMessagePostRequestReplyMarkup>>,
}

impl SendPollPostRequest {
    pub fn new(chat_id: models::SendMessagePostRequestChatId, question: String, options: Vec<models::InputPollOption>) -> SendPollPostRequest {
        SendPollPostRequest {
            business_connection_id: None,
            chat_id: Box::new(chat_id),
            message_thread_id: None,
            question,
            question_parse_mode: None,
            question_entities: None,
            options,
            is_anonymous: None,
            r#type: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }
}
/// Poll type, “quiz” or “regular”, defaults to “regular”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "quiz")]
    Quiz,
    #[serde(rename = "regular")]
    Regular,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Quiz
    }
}

