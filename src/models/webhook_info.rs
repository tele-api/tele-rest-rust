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

/// WebhookInfo : Describes the current status of a webhook.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    #[serde(rename = "url")]
    pub url: String,
    /// *True*, if a custom certificate was provided for webhook certificate checks
    #[serde(rename = "has_custom_certificate")]
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    #[serde(rename = "pending_update_count")]
    pub pending_update_count: i32,
    /// *Optional*. Currently used webhook IP address
    #[serde(rename = "ip_address", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// *Optional*. Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(rename = "last_error_date", skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i32>,
    /// *Optional*. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(rename = "last_error_message", skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// *Optional*. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(rename = "last_synchronization_error_date", skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i32>,
    /// *Optional*. The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(rename = "max_connections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// *Optional*. A list of update types the bot is subscribed to. Defaults to all update types except *chat\\_member*
    #[serde(rename = "allowed_updates", skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl WebhookInfo {
    /// Describes the current status of a webhook.
    pub fn new(url: String, has_custom_certificate: bool, pending_update_count: i32) -> WebhookInfo {
        WebhookInfo {
            url,
            has_custom_certificate,
            pending_update_count,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            last_synchronization_error_date: None,
            max_connections: None,
            allowed_updates: None,
        }
    }
}

