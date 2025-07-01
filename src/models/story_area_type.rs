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

/// StoryAreaType : Describes the type of a clickable area on a story. Currently, it can be one of  * [StoryAreaTypeLocation](https://core.telegram.org/bots/api/#storyareatypelocation) * [StoryAreaTypeSuggestedReaction](https://core.telegram.org/bots/api/#storyareatypesuggestedreaction) * [StoryAreaTypeLink](https://core.telegram.org/bots/api/#storyareatypelink) * [StoryAreaTypeWeather](https://core.telegram.org/bots/api/#storyareatypeweather) * [StoryAreaTypeUniqueGift](https://core.telegram.org/bots/api/#storyareatypeuniquegift)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaType {
    /// Type of the area, always “unique\\_gift”
    #[serde(rename = "type")]
    pub r#type: String,
    /// Location latitude in degrees
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Location longitude in degrees
    #[serde(rename = "longitude")]
    pub longitude: f64,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::LocationAddress>>,
    #[serde(rename = "reaction_type")]
    pub reaction_type: Box<models::ReactionType>,
    /// *Optional*. Pass *True* if the reaction area has a dark background
    #[serde(rename = "is_dark", skip_serializing_if = "Option::is_none")]
    pub is_dark: Option<bool>,
    /// *Optional*. Pass *True* if reaction area corner is flipped
    #[serde(rename = "is_flipped", skip_serializing_if = "Option::is_none")]
    pub is_flipped: Option<bool>,
    /// HTTP or tg:// URL to be opened when the area is clicked
    #[serde(rename = "url")]
    pub url: String,
    /// Temperature, in degree Celsius
    #[serde(rename = "temperature")]
    pub temperature: f64,
    /// Emoji representing the weather
    #[serde(rename = "emoji")]
    pub emoji: String,
    /// A color of the area background in the ARGB format
    #[serde(rename = "background_color")]
    pub background_color: i32,
    /// Unique name of the gift
    #[serde(rename = "name")]
    pub name: String,
}

impl StoryAreaType {
    /// Describes the type of a clickable area on a story. Currently, it can be one of  * [StoryAreaTypeLocation](https://core.telegram.org/bots/api/#storyareatypelocation) * [StoryAreaTypeSuggestedReaction](https://core.telegram.org/bots/api/#storyareatypesuggestedreaction) * [StoryAreaTypeLink](https://core.telegram.org/bots/api/#storyareatypelink) * [StoryAreaTypeWeather](https://core.telegram.org/bots/api/#storyareatypeweather) * [StoryAreaTypeUniqueGift](https://core.telegram.org/bots/api/#storyareatypeuniquegift)
    pub fn new(r#type: String, latitude: f64, longitude: f64, reaction_type: models::ReactionType, url: String, temperature: f64, emoji: String, background_color: i32, name: String) -> StoryAreaType {
        StoryAreaType {
            r#type,
            latitude,
            longitude,
            address: None,
            reaction_type: Box::new(reaction_type),
            is_dark: None,
            is_flipped: None,
            url,
            temperature,
            emoji,
            background_color,
            name,
        }
    }
}

