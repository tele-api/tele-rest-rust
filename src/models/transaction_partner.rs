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

/// TransactionPartner : This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of  * [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser) * [TransactionPartnerChat](https://core.telegram.org/bots/api/#transactionpartnerchat) * [TransactionPartnerAffiliateProgram](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram) * [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment) * [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads) * [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi) * [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartner {
    /// Type of the transaction partner, always “other”
    #[serde(rename = "type")]
    pub r#type: String,
    /// Type of the transaction, currently one of “invoice\\_payment” for payments via invoices, “paid\\_media\\_payment” for payments for paid media, “gift\\_purchase” for gifts sent by the bot, “premium\\_purchase” for Telegram Premium subscriptions gifted by the bot, “business\\_account\\_transfer” for direct transfers from managed business accounts
    #[serde(rename = "transaction_type")]
    pub transaction_type: TransactionTypeEnum,
    #[serde(rename = "user")]
    pub user: Box<models::User>,
    #[serde(rename = "affiliate", skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<Box<models::AffiliateInfo>>,
    /// *Optional*. Bot-specified invoice payload. Can be available only for “invoice\\_payment” transactions.
    #[serde(rename = "invoice_payload", skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
    /// *Optional*. The duration of the paid subscription. Can be available only for “invoice\\_payment” transactions.
    #[serde(rename = "subscription_period", skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i32>,
    /// *Optional*. Information about the paid media bought by the user; for “paid\\_media\\_payment” transactions only
    #[serde(rename = "paid_media", skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<models::PaidMedia>>,
    /// *Optional*. Bot-specified paid media payload. Can be available only for “paid\\_media\\_payment” transactions.
    #[serde(rename = "paid_media_payload", skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,
    #[serde(rename = "gift", skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<models::Gift>>,
    /// *Optional*. Number of months the gifted Telegram Premium subscription will be active for; for “premium\\_purchase” transactions only
    #[serde(rename = "premium_subscription_duration", skip_serializing_if = "Option::is_none")]
    pub premium_subscription_duration: Option<i32>,
    #[serde(rename = "chat")]
    pub chat: Box<models::Chat>,
    #[serde(rename = "sponsor_user", skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<Box<models::User>>,
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    #[serde(rename = "commission_per_mille")]
    pub commission_per_mille: i32,
    #[serde(rename = "withdrawal_state", skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<Box<models::RevenueWithdrawalState>>,
    /// The number of successful requests that exceeded regular limits and were therefore billed
    #[serde(rename = "request_count")]
    pub request_count: i32,
}

impl TransactionPartner {
    /// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of  * [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser) * [TransactionPartnerChat](https://core.telegram.org/bots/api/#transactionpartnerchat) * [TransactionPartnerAffiliateProgram](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram) * [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment) * [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads) * [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi) * [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
    pub fn new(r#type: String, transaction_type: TransactionTypeEnum, user: models::User, chat: models::Chat, commission_per_mille: i32, request_count: i32) -> TransactionPartner {
        TransactionPartner {
            r#type,
            transaction_type,
            user: Box::new(user),
            affiliate: None,
            invoice_payload: None,
            subscription_period: None,
            paid_media: None,
            paid_media_payload: None,
            gift: None,
            premium_subscription_duration: None,
            chat: Box::new(chat),
            sponsor_user: None,
            commission_per_mille,
            withdrawal_state: None,
            request_count,
        }
    }
}
/// Type of the transaction, currently one of “invoice\\_payment” for payments via invoices, “paid\\_media\\_payment” for payments for paid media, “gift\\_purchase” for gifts sent by the bot, “premium\\_purchase” for Telegram Premium subscriptions gifted by the bot, “business\\_account\\_transfer” for direct transfers from managed business accounts
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionTypeEnum {
    #[serde(rename = "invoice_payment")]
    InvoicePayment,
    #[serde(rename = "paid_media_payment")]
    PaidMediaPayment,
    #[serde(rename = "gift_purchase")]
    GiftPurchase,
    #[serde(rename = "premium_purchase")]
    PremiumPurchase,
    #[serde(rename = "business_account_transfer")]
    BusinessAccountTransfer,
}

impl Default for TransactionTypeEnum {
    fn default() -> TransactionTypeEnum {
        Self::InvoicePayment
    }
}

