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

/// Giveaway : This object represents a message about a scheduled giveaway.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway
    #[serde(rename = "chats")]
    pub chats: Vec<models::Chat>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    #[serde(rename = "winners_selection_date")]
    pub winners_selection_date: i32,
    /// The number of users which are supposed to be selected as winners of the giveaway
    #[serde(rename = "winner_count")]
    pub winner_count: i32,
    /// *Optional*. *True*, if only users who join the chats after the giveaway started should be eligible to win
    #[serde(rename = "only_new_members", skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// *Optional*. *True*, if the list of giveaway winners will be visible to everyone
    #[serde(rename = "has_public_winners", skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    /// *Optional*. Description of additional giveaway prize
    #[serde(rename = "prize_description", skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    /// *Optional*. A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[serde(rename = "country_codes", skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(rename = "prize_star_count", skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i32>,
    /// *Optional*. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[serde(rename = "premium_subscription_month_count", skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i32>,
}

impl Giveaway {
    /// This object represents a message about a scheduled giveaway.
    pub fn new(chats: Vec<models::Chat>, winners_selection_date: i32, winner_count: i32) -> Giveaway {
        Giveaway {
            chats,
            winners_selection_date,
            winner_count,
            only_new_members: None,
            has_public_winners: None,
            prize_description: None,
            country_codes: None,
            prize_star_count: None,
            premium_subscription_month_count: None,
        }
    }
}

