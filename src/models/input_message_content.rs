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

/// InputMessageContent : This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:  * [InputTextMessageContent](https://core.telegram.org/bots/api/#inputtextmessagecontent) * [InputLocationMessageContent](https://core.telegram.org/bots/api/#inputlocationmessagecontent) * [InputVenueMessageContent](https://core.telegram.org/bots/api/#inputvenuemessagecontent) * [InputContactMessageContent](https://core.telegram.org/bots/api/#inputcontactmessagecontent) * [InputInvoiceMessageContent](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    #[serde(rename = "message_text")]
    pub message_text: String,
    /// *Optional*. Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "parse_mode", skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in message text, which can be specified instead of *parse\\_mode*
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<models::MessageEntity>>,
    #[serde(rename = "link_preview_options", skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<Box<models::LinkPreviewOptions>>,
    /// Latitude of the venue in degrees
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Longitude of the venue in degrees
    #[serde(rename = "longitude")]
    pub longitude: f64,
    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(rename = "horizontal_accuracy", skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// *Optional*. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[serde(rename = "live_period", skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i32>,
    /// *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(rename = "heading", skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,
    /// *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(rename = "proximity_alert_radius", skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
    /// Product name, 1-32 characters
    #[serde(rename = "title")]
    pub title: String,
    /// Address of the venue
    #[serde(rename = "address")]
    pub address: String,
    /// *Optional*. Foursquare identifier of the venue, if known
    #[serde(rename = "foursquare_id", skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// *Optional*. Foursquare type of the venue, if known. (For example, “arts\\_entertainment/default”, “arts\\_entertainment/aquarium” or “food/icecream”.)
    #[serde(rename = "foursquare_type", skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// *Optional*. Google Places identifier of the venue
    #[serde(rename = "google_place_id", skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
    #[serde(rename = "google_place_type", skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Contact's phone number
    #[serde(rename = "phone_number")]
    pub phone_number: String,
    /// Contact's first name
    #[serde(rename = "first_name")]
    pub first_name: String,
    /// *Optional*. Contact's last name
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    #[serde(rename = "vcard", skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Product description, 1-255 characters
    #[serde(rename = "description")]
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[serde(rename = "payload")]
    pub payload: String,
    /// *Optional*. Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "provider_token", skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "currency")]
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "prices")]
    pub prices: Vec<models::LabeledPrice>,
    /// *Optional*. The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "max_tip_amount", skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i32>,
    /// *Optional*. A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\\_tip\\_amount*.
    #[serde(rename = "suggested_tip_amounts", skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i32>>,
    /// *Optional*. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[serde(rename = "provider_data", skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// *Optional*. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(rename = "photo_url", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// *Optional*. Photo size in bytes
    #[serde(rename = "photo_size", skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i32>,
    /// *Optional*. Photo width
    #[serde(rename = "photo_width", skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,
    /// *Optional*. Photo height
    #[serde(rename = "photo_height", skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,
    /// *Optional*. Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_name", skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// *Optional*. Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_phone_number", skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// *Optional*. Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_email", skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// *Optional*. Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "need_shipping_address", skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// *Optional*. Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "send_phone_number_to_provider", skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// *Optional*. Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "send_email_to_provider", skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// *Optional*. Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(rename = "is_flexible", skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

impl InputMessageContent {
    /// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:  * [InputTextMessageContent](https://core.telegram.org/bots/api/#inputtextmessagecontent) * [InputLocationMessageContent](https://core.telegram.org/bots/api/#inputlocationmessagecontent) * [InputVenueMessageContent](https://core.telegram.org/bots/api/#inputvenuemessagecontent) * [InputContactMessageContent](https://core.telegram.org/bots/api/#inputcontactmessagecontent) * [InputInvoiceMessageContent](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)
    pub fn new(message_text: String, latitude: f64, longitude: f64, title: String, address: String, phone_number: String, first_name: String, description: String, payload: String, currency: String, prices: Vec<models::LabeledPrice>) -> InputMessageContent {
        InputMessageContent {
            message_text,
            parse_mode: None,
            entities: None,
            link_preview_options: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            description,
            payload,
            provider_token: None,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
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
        }
    }
}

