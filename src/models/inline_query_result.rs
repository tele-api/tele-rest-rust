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

/// InlineQueryResult : This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:  * [InlineQueryResultCachedAudio](https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio) * [InlineQueryResultCachedDocument](https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument) * [InlineQueryResultCachedGif](https://core.telegram.org/bots/api/#inlinequeryresultcachedgif) * [InlineQueryResultCachedMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif) * [InlineQueryResultCachedPhoto](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto) * [InlineQueryResultCachedSticker](https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker) * [InlineQueryResultCachedVideo](https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo) * [InlineQueryResultCachedVoice](https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice) * [InlineQueryResultArticle](https://core.telegram.org/bots/api/#inlinequeryresultarticle) * [InlineQueryResultAudio](https://core.telegram.org/bots/api/#inlinequeryresultaudio) * [InlineQueryResultContact](https://core.telegram.org/bots/api/#inlinequeryresultcontact) * [InlineQueryResultGame](https://core.telegram.org/bots/api/#inlinequeryresultgame) * [InlineQueryResultDocument](https://core.telegram.org/bots/api/#inlinequeryresultdocument) * [InlineQueryResultGif](https://core.telegram.org/bots/api/#inlinequeryresultgif) * [InlineQueryResultLocation](https://core.telegram.org/bots/api/#inlinequeryresultlocation) * [InlineQueryResultMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif) * [InlineQueryResultPhoto](https://core.telegram.org/bots/api/#inlinequeryresultphoto) * [InlineQueryResultVenue](https://core.telegram.org/bots/api/#inlinequeryresultvenue) * [InlineQueryResultVideo](https://core.telegram.org/bots/api/#inlinequeryresultvideo) * [InlineQueryResultVoice](https://core.telegram.org/bots/api/#inlinequeryresultvoice)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResult {
    /// Type of the result, must be *voice*
    #[serde(rename = "type")]
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    #[serde(rename = "id")]
    pub id: String,
    /// A valid file identifier for the audio file
    #[serde(rename = "audio_file_id")]
    pub audio_file_id: String,
    /// *Optional*. Caption, 0-1024 characters after entities parsing
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(rename = "parse_mode", skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode*
    #[serde(rename = "caption_entities", skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<models::MessageEntity>>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<models::InlineKeyboardMarkup>>,
    #[serde(rename = "input_message_content")]
    pub input_message_content: Box<models::InputMessageContent>,
    /// Recording title
    #[serde(rename = "title")]
    pub title: String,
    /// A valid file identifier for the file
    #[serde(rename = "document_file_id")]
    pub document_file_id: String,
    /// *Optional*. Short description of the result
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A valid file identifier for the GIF file
    #[serde(rename = "gif_file_id")]
    pub gif_file_id: String,
    /// *Optional*. Pass *True*, if the caption must be shown above the message media
    #[serde(rename = "show_caption_above_media", skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// A valid file identifier for the MPEG4 file
    #[serde(rename = "mpeg4_file_id")]
    pub mpeg4_file_id: String,
    /// A valid file identifier of the photo
    #[serde(rename = "photo_file_id")]
    pub photo_file_id: String,
    /// A valid file identifier of the sticker
    #[serde(rename = "sticker_file_id")]
    pub sticker_file_id: String,
    /// A valid file identifier for the video file
    #[serde(rename = "video_file_id")]
    pub video_file_id: String,
    /// A valid file identifier for the voice message
    #[serde(rename = "voice_file_id")]
    pub voice_file_id: String,
    /// *Optional*. URL of the result
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URL of the thumbnail (JPEG only) for the video
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    /// *Optional*. Thumbnail width
    #[serde(rename = "thumbnail_width", skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,
    /// *Optional*. Thumbnail height
    #[serde(rename = "thumbnail_height", skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,
    /// A valid URL for the audio file
    #[serde(rename = "audio_url")]
    pub audio_url: String,
    /// *Optional*. Performer
    #[serde(rename = "performer", skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// *Optional*. Audio duration in seconds
    #[serde(rename = "audio_duration", skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i32>,
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
    /// Short name of the game
    #[serde(rename = "game_short_name")]
    pub game_short_name: String,
    /// A valid URL for the file
    #[serde(rename = "document_url")]
    pub document_url: String,
    /// MIME type of the content of the video URL, “text/html” or “video/mp4”
    #[serde(rename = "mime_type")]
    pub mime_type: MimeTypeEnum,
    /// A valid URL for the GIF file
    #[serde(rename = "gif_url")]
    pub gif_url: String,
    /// *Optional*. Width of the GIF
    #[serde(rename = "gif_width", skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i32>,
    /// *Optional*. Height of the GIF
    #[serde(rename = "gif_height", skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i32>,
    /// *Optional*. Duration of the GIF in seconds
    #[serde(rename = "gif_duration", skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i32>,
    /// *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(rename = "thumbnail_mime_type", skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<ThumbnailMimeTypeEnum>,
    /// Latitude of the venue location in degrees
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Longitude of the venue location in degrees
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
    /// A valid URL for the MPEG4 file
    #[serde(rename = "mpeg4_url")]
    pub mpeg4_url: String,
    /// *Optional*. Video width
    #[serde(rename = "mpeg4_width", skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i32>,
    /// *Optional*. Video height
    #[serde(rename = "mpeg4_height", skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i32>,
    /// *Optional*. Video duration in seconds
    #[serde(rename = "mpeg4_duration", skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i32>,
    /// A valid URL of the photo. Photo must be in **JPEG** format. Photo size must not exceed 5MB
    #[serde(rename = "photo_url")]
    pub photo_url: String,
    /// *Optional*. Width of the photo
    #[serde(rename = "photo_width", skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,
    /// *Optional*. Height of the photo
    #[serde(rename = "photo_height", skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,
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
    /// A valid URL for the embedded video player or video file
    #[serde(rename = "video_url")]
    pub video_url: String,
    /// *Optional*. Video width
    #[serde(rename = "video_width", skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i32>,
    /// *Optional*. Video height
    #[serde(rename = "video_height", skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i32>,
    /// *Optional*. Video duration in seconds
    #[serde(rename = "video_duration", skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i32>,
    /// A valid URL for the voice recording
    #[serde(rename = "voice_url")]
    pub voice_url: String,
    /// *Optional*. Recording duration in seconds
    #[serde(rename = "voice_duration", skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i32>,
}

impl InlineQueryResult {
    /// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:  * [InlineQueryResultCachedAudio](https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio) * [InlineQueryResultCachedDocument](https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument) * [InlineQueryResultCachedGif](https://core.telegram.org/bots/api/#inlinequeryresultcachedgif) * [InlineQueryResultCachedMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif) * [InlineQueryResultCachedPhoto](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto) * [InlineQueryResultCachedSticker](https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker) * [InlineQueryResultCachedVideo](https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo) * [InlineQueryResultCachedVoice](https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice) * [InlineQueryResultArticle](https://core.telegram.org/bots/api/#inlinequeryresultarticle) * [InlineQueryResultAudio](https://core.telegram.org/bots/api/#inlinequeryresultaudio) * [InlineQueryResultContact](https://core.telegram.org/bots/api/#inlinequeryresultcontact) * [InlineQueryResultGame](https://core.telegram.org/bots/api/#inlinequeryresultgame) * [InlineQueryResultDocument](https://core.telegram.org/bots/api/#inlinequeryresultdocument) * [InlineQueryResultGif](https://core.telegram.org/bots/api/#inlinequeryresultgif) * [InlineQueryResultLocation](https://core.telegram.org/bots/api/#inlinequeryresultlocation) * [InlineQueryResultMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif) * [InlineQueryResultPhoto](https://core.telegram.org/bots/api/#inlinequeryresultphoto) * [InlineQueryResultVenue](https://core.telegram.org/bots/api/#inlinequeryresultvenue) * [InlineQueryResultVideo](https://core.telegram.org/bots/api/#inlinequeryresultvideo) * [InlineQueryResultVoice](https://core.telegram.org/bots/api/#inlinequeryresultvoice)
    pub fn new(r#type: String, id: String, audio_file_id: String, input_message_content: models::InputMessageContent, title: String, document_file_id: String, gif_file_id: String, mpeg4_file_id: String, photo_file_id: String, sticker_file_id: String, video_file_id: String, voice_file_id: String, thumbnail_url: String, audio_url: String, phone_number: String, first_name: String, game_short_name: String, document_url: String, mime_type: MimeTypeEnum, gif_url: String, latitude: f64, longitude: f64, mpeg4_url: String, photo_url: String, address: String, video_url: String, voice_url: String) -> InlineQueryResult {
        InlineQueryResult {
            r#type,
            id,
            audio_file_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: Box::new(input_message_content),
            title,
            document_file_id,
            description: None,
            gif_file_id,
            show_caption_above_media: None,
            mpeg4_file_id,
            photo_file_id,
            sticker_file_id,
            video_file_id,
            voice_file_id,
            url: None,
            thumbnail_url,
            thumbnail_width: None,
            thumbnail_height: None,
            audio_url,
            performer: None,
            audio_duration: None,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            game_short_name,
            document_url,
            mime_type,
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumbnail_mime_type: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            mpeg4_url,
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            photo_url,
            photo_width: None,
            photo_height: None,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            video_url,
            video_width: None,
            video_height: None,
            video_duration: None,
            voice_url,
            voice_duration: None,
        }
    }
}
/// MIME type of the content of the video URL, “text/html” or “video/mp4”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MimeTypeEnum {
    #[serde(rename = "text/html")]
    TextSlashHtml,
    #[serde(rename = "video/mp4")]
    VideoSlashMp4,
}

impl Default for MimeTypeEnum {
    fn default() -> MimeTypeEnum {
        Self::TextSlashHtml
    }
}
/// *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ThumbnailMimeTypeEnum {
    #[serde(rename = "image/jpeg")]
    ImageSlashJpeg,
    #[serde(rename = "image/gif")]
    ImageSlashGif,
    #[serde(rename = "video/mp4")]
    VideoSlashMp4,
}

impl Default for ThumbnailMimeTypeEnum {
    fn default() -> ThumbnailMimeTypeEnum {
        Self::ImageSlashJpeg
    }
}

