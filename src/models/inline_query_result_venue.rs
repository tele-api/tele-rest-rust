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

/// InlineQueryResultVenue : Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use *input\\_message\\_content* to send a message with the specified content instead of the venue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be *venue*
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    #[serde(rename = "id")]
    pub id: String,
    /// Latitude of the venue location in degrees
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    #[serde(rename = "longitude")]
    pub longitude: f64,
    /// Title of the venue
    #[serde(rename = "title")]
    pub title: String,
    /// Address of the venue
    #[serde(rename = "address")]
    pub address: String,
    /// *Optional*. Foursquare identifier of the venue if known
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
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
    #[serde(rename = "input_message_content", skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<Box<models::InputMessageContent>>,
    /// *Optional*. Url of the thumbnail for the result
    #[serde(rename = "thumbnail_url", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// *Optional*. Thumbnail width
    #[serde(rename = "thumbnail_width", skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,
    /// *Optional*. Thumbnail height
    #[serde(rename = "thumbnail_height", skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,
}

impl InlineQueryResultVenue {
    /// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use *input\\_message\\_content* to send a message with the specified content instead of the venue.
    pub fn new(r#type: String, id: String, latitude: f64, longitude: f64, title: String, address: String) -> InlineQueryResultVenue {
        InlineQueryResultVenue {
            r#type,
            id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }
}

