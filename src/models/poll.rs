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

/// Poll : This object contains information about a poll.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Poll question, 1-300 characters
    #[serde(rename = "question")]
    pub question: String,
    /// *Optional*. Special entities that appear in the *question*. Currently, only custom emoji entities are allowed in poll questions
    #[serde(rename = "question_entities", skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<models::MessageEntity>>,
    /// List of poll options
    #[serde(rename = "options")]
    pub options: Vec<models::PollOption>,
    /// Total number of users that voted in the poll
    #[serde(rename = "total_voter_count")]
    pub total_voter_count: i32,
    /// *True*, if the poll is closed
    #[serde(rename = "is_closed")]
    pub is_closed: bool,
    /// *True*, if the poll is anonymous
    #[serde(rename = "is_anonymous")]
    pub is_anonymous: bool,
    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// *True*, if the poll allows multiple answers
    #[serde(rename = "allows_multiple_answers")]
    pub allows_multiple_answers: bool,
    /// *Optional*. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    #[serde(rename = "correct_option_id", skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i32>,
    /// *Optional*. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(rename = "explanation", skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation*
    #[serde(rename = "explanation_entities", skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<models::MessageEntity>>,
    /// *Optional*. Amount of time in seconds the poll will be active after creation
    #[serde(rename = "open_period", skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i32>,
    /// *Optional*. Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(rename = "close_date", skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i32>,
}

impl Poll {
    /// This object contains information about a poll.
    pub fn new(id: String, question: String, options: Vec<models::PollOption>, total_voter_count: i32, is_closed: bool, is_anonymous: bool, r#type: TypeEnum, allows_multiple_answers: bool) -> Poll {
        Poll {
            id,
            question,
            question_entities: None,
            options,
            total_voter_count,
            is_closed,
            is_anonymous,
            r#type,
            allows_multiple_answers,
            correct_option_id: None,
            explanation: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }
}
/// Poll type, currently can be “regular” or “quiz”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "quiz")]
    Quiz,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Regular
    }
}

