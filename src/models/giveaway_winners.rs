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

/// GiveawayWinners : This object represents a message about the completion of a giveaway with public winners.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveawayWinners {
    #[serde(rename = "chat")]
    pub chat: Box<models::Chat>,
    /// Identifier of the message with the giveaway in the chat
    #[serde(rename = "giveaway_message_id")]
    pub giveaway_message_id: i32,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    #[serde(rename = "winners_selection_date")]
    pub winners_selection_date: i32,
    /// Total number of winners in the giveaway
    #[serde(rename = "winner_count")]
    pub winner_count: i32,
    /// List of up to 100 winners of the giveaway
    #[serde(rename = "winners")]
    pub winners: Vec<models::User>,
    /// *Optional*. The number of other chats the user had to join in order to be eligible for the giveaway
    #[serde(rename = "additional_chat_count", skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i32>,
    /// *Optional*. The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    #[serde(rename = "prize_star_count", skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i32>,
    /// *Optional*. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[serde(rename = "premium_subscription_month_count", skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i32>,
    /// *Optional*. Number of undistributed prizes
    #[serde(rename = "unclaimed_prize_count", skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i32>,
    /// *Optional*. *True*, if only users who had joined the chats after the giveaway started were eligible to win
    #[serde(rename = "only_new_members", skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// *Optional*. *True*, if the giveaway was canceled because the payment for it was refunded
    #[serde(rename = "was_refunded", skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// *Optional*. Description of additional giveaway prize
    #[serde(rename = "prize_description", skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

impl GiveawayWinners {
    /// This object represents a message about the completion of a giveaway with public winners.
    pub fn new(chat: models::Chat, giveaway_message_id: i32, winners_selection_date: i32, winner_count: i32, winners: Vec<models::User>) -> GiveawayWinners {
        GiveawayWinners {
            chat: Box::new(chat),
            giveaway_message_id,
            winners_selection_date,
            winner_count,
            winners,
            additional_chat_count: None,
            prize_star_count: None,
            premium_subscription_month_count: None,
            unclaimed_prize_count: None,
            only_new_members: None,
            was_refunded: None,
            prize_description: None,
        }
    }
}

