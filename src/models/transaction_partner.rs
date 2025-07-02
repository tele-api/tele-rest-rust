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

/// TransactionPartner : This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of  * [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser) * [TransactionPartnerChat](https://core.telegram.org/bots/api/#transactionpartnerchat) * [TransactionPartnerAffiliateProgram](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram) * [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment) * [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads) * [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi) * [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of  * [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser) * [TransactionPartnerChat](https://core.telegram.org/bots/api/#transactionpartnerchat) * [TransactionPartnerAffiliateProgram](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram) * [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment) * [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads) * [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi) * [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionPartner {
    TransactionPartnerUser(Box<models::TransactionPartnerUser>),
    TransactionPartnerChat(Box<models::TransactionPartnerChat>),
    TransactionPartnerAffiliateProgram(Box<models::TransactionPartnerAffiliateProgram>),
    TransactionPartnerFragment(Box<models::TransactionPartnerFragment>),
    TransactionPartnerTelegramAds(Box<models::TransactionPartnerTelegramAds>),
    TransactionPartnerTelegramApi(Box<models::TransactionPartnerTelegramApi>),
    TransactionPartnerOther(Box<models::TransactionPartnerOther>),
}

impl Default for TransactionPartner {
    fn default() -> Self {
        Self::TransactionPartnerUser(Default::default())
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

