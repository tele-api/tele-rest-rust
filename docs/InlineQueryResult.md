# InlineQueryResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *voice* | [default to voice]
**id** | **String** | Unique identifier for this result, 1-64 bytes | 
**audio_file_id** | **String** | A valid file identifier for the audio file | 
**caption** | Option<**String**> | *Optional*. Caption, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | [**models::InputMessageContent**](InputMessageContent.md) |  | 
**title** | **String** | Recording title | 
**document_file_id** | **String** | A valid file identifier for the file | 
**description** | Option<**String**> | *Optional*. Short description of the result | [optional]
**gif_file_id** | **String** | A valid file identifier for the GIF file | 
**show_caption_above_media** | Option<**bool**> | *Optional*. Pass *True*, if the caption must be shown above the message media | [optional]
**mpeg4_file_id** | **String** | A valid file identifier for the MPEG4 file | 
**photo_file_id** | **String** | A valid file identifier of the photo | 
**sticker_file_id** | **String** | A valid file identifier of the sticker | 
**video_file_id** | **String** | A valid file identifier for the video file | 
**voice_file_id** | **String** | A valid file identifier for the voice message | 
**url** | Option<**String**> | *Optional*. URL of the result | [optional]
**thumbnail_url** | **String** | URL of the thumbnail (JPEG only) for the video | 
**thumbnail_width** | Option<**i32**> | *Optional*. Thumbnail width | [optional]
**thumbnail_height** | Option<**i32**> | *Optional*. Thumbnail height | [optional]
**audio_url** | **String** | A valid URL for the audio file | 
**performer** | Option<**String**> | *Optional*. Performer | [optional]
**audio_duration** | Option<**i32**> | *Optional*. Audio duration in seconds | [optional]
**phone_number** | **String** | Contact's phone number | 
**first_name** | **String** | Contact's first name | 
**last_name** | Option<**String**> | *Optional*. Contact's last name | [optional]
**vcard** | Option<**String**> | *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes | [optional]
**game_short_name** | **String** | Short name of the game | 
**document_url** | **String** | A valid URL for the file | 
**mime_type** | **String** | MIME type of the content of the video URL, “text/html” or “video/mp4” | 
**gif_url** | **String** | A valid URL for the GIF file | 
**gif_width** | Option<**i32**> | *Optional*. Width of the GIF | [optional]
**gif_height** | Option<**i32**> | *Optional*. Height of the GIF | [optional]
**gif_duration** | Option<**i32**> | *Optional*. Duration of the GIF in seconds | [optional]
**thumbnail_mime_type** | Option<**String**> | *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg” | [optional][default to ImageSlashJpeg]
**latitude** | **f64** | Latitude of the venue location in degrees | 
**longitude** | **f64** | Longitude of the venue location in degrees | 
**horizontal_accuracy** | Option<**f64**> | *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500 | [optional]
**live_period** | Option<**i32**> | *Optional*. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely. | [optional]
**heading** | Option<**i32**> | *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified. | [optional]
**proximity_alert_radius** | Option<**i32**> | *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified. | [optional]
**mpeg4_url** | **String** | A valid URL for the MPEG4 file | 
**mpeg4_width** | Option<**i32**> | *Optional*. Video width | [optional]
**mpeg4_height** | Option<**i32**> | *Optional*. Video height | [optional]
**mpeg4_duration** | Option<**i32**> | *Optional*. Video duration in seconds | [optional]
**photo_url** | **String** | A valid URL of the photo. Photo must be in **JPEG** format. Photo size must not exceed 5MB | 
**photo_width** | Option<**i32**> | *Optional*. Width of the photo | [optional]
**photo_height** | Option<**i32**> | *Optional*. Height of the photo | [optional]
**address** | **String** | Address of the venue | 
**foursquare_id** | Option<**String**> | *Optional*. Foursquare identifier of the venue if known | [optional]
**foursquare_type** | Option<**String**> | *Optional*. Foursquare type of the venue, if known. (For example, “arts\\_entertainment/default”, “arts\\_entertainment/aquarium” or “food/icecream”.) | [optional]
**google_place_id** | Option<**String**> | *Optional*. Google Places identifier of the venue | [optional]
**google_place_type** | Option<**String**> | *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).) | [optional]
**video_url** | **String** | A valid URL for the embedded video player or video file | 
**video_width** | Option<**i32**> | *Optional*. Video width | [optional]
**video_height** | Option<**i32**> | *Optional*. Video height | [optional]
**video_duration** | Option<**i32**> | *Optional*. Video duration in seconds | [optional]
**voice_url** | **String** | A valid URL for the voice recording | 
**voice_duration** | Option<**i32**> | *Optional*. Recording duration in seconds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


