# InlineQueryResultMpeg4Gif

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *mpeg4\\_gif* | [default to mpeg4_gif]
**id** | **String** | Unique identifier for this result, 1-64 bytes | 
**mpeg4_url** | **String** | A valid URL for the MPEG4 file | 
**mpeg4_width** | Option<**i32**> | *Optional*. Video width | [optional]
**mpeg4_height** | Option<**i32**> | *Optional*. Video height | [optional]
**mpeg4_duration** | Option<**i32**> | *Optional*. Video duration in seconds | [optional]
**thumbnail_url** | **String** | URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result | 
**thumbnail_mime_type** | Option<**String**> | *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg” | [optional][default to ImageSlashJpeg]
**title** | Option<**String**> | *Optional*. Title for the result | [optional]
**caption** | Option<**String**> | *Optional*. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**show_caption_above_media** | Option<**bool**> | *Optional*. Pass *True*, if the caption must be shown above the message media | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


