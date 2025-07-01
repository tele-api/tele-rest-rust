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
pub struct AnswerInlineQueryPostRequest {
    /// Unique identifier for the answered query
    #[serde(rename = "inline_query_id")]
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    #[serde(rename = "results")]
    pub results: Vec<models::InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(rename = "cache_time", skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[serde(rename = "is_personal", skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(rename = "next_offset", skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    #[serde(rename = "button", skip_serializing_if = "Option::is_none")]
    pub button: Option<Box<models::InlineQueryResultsButton>>,
}

impl AnswerInlineQueryPostRequest {
    pub fn new(inline_query_id: String, results: Vec<models::InlineQueryResult>) -> AnswerInlineQueryPostRequest {
        AnswerInlineQueryPostRequest {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            button: None,
        }
    }
}

