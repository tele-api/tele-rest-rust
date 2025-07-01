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

/// PreCheckoutQuery : This object contains information about an incoming pre-checkout query.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "from")]
    pub from: Box<models::User>,
    /// Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)
    #[serde(rename = "currency")]
    pub currency: String,
    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    #[serde(rename = "total_amount")]
    pub total_amount: i32,
    /// Bot-specified invoice payload
    #[serde(rename = "invoice_payload")]
    pub invoice_payload: String,
    /// *Optional*. Identifier of the shipping option chosen by the user
    #[serde(rename = "shipping_option_id", skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(rename = "order_info", skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Box<models::OrderInfo>>,
}

impl PreCheckoutQuery {
    /// This object contains information about an incoming pre-checkout query.
    pub fn new(id: String, from: models::User, currency: String, total_amount: i32, invoice_payload: String) -> PreCheckoutQuery {
        PreCheckoutQuery {
            id,
            from: Box::new(from),
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
        }
    }
}

