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

/// SendInvoiceRequest : Request parameters for sendInvoice
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendInvoiceRequest {
    #[serde(rename = "chat_id")]
    pub chat_id: Box<models::SendMessageRequestChatId>,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(rename = "message_thread_id", skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// Product name, 1-32 characters
    #[serde(rename = "title")]
    pub title: String,
    /// Product description, 1-255 characters
    #[serde(rename = "description")]
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[serde(rename = "payload")]
    pub payload: String,
    /// Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "provider_token", skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "currency")]
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "prices")]
    pub prices: Vec<models::LabeledPrice>,
    /// The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "max_tip_amount", skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i32>,
    /// A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\\_tip\\_amount*.
    #[serde(rename = "suggested_tip_amounts", skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i32>>,
    /// Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a *Pay* button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a *URL* button with a deep link to the bot (instead of a *Pay* button), with the value used as the start parameter
    #[serde(rename = "start_parameter", skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(rename = "provider_data", skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(rename = "photo_url", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(rename = "photo_size", skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i32>,
    /// Photo width
    #[serde(rename = "photo_width", skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,
    /// Photo height
    #[serde(rename = "photo_height", skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,
    /// Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_name", skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_phone_number", skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_email", skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_shipping_address", skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "send_phone_number_to_provider", skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "send_email_to_provider", skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "is_flexible", skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
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
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
}

impl SendInvoiceRequest {
    /// Request parameters for sendInvoice
    pub fn new(chat_id: models::SendMessageRequestChatId, title: String, description: String, payload: String, currency: String, prices: Vec<models::LabeledPrice>) -> SendInvoiceRequest {
        SendInvoiceRequest {
            chat_id: Box::new(chat_id),
            message_thread_id: None,
            title,
            description,
            payload,
            provider_token: None,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }
}

