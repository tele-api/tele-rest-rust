# InlineQueryResultGif

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *gif* | [default to gif]
**id** | **String** | Unique identifier for this result, 1-64 bytes | 
**gif_url** | **String** | A valid URL for the GIF file | 
**gif_width** | Option<**i32**> | *Optional*. Width of the GIF | [optional]
**gif_height** | Option<**i32**> | *Optional*. Height of the GIF | [optional]
**gif_duration** | Option<**i32**> | *Optional*. Duration of the GIF in seconds | [optional]
**thumbnail_url** | **String** | URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result | 
**thumbnail_mime_type** | Option<**String**> | *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg” | [optional][default to ImageSlashJpeg]
**title** | Option<**String**> | *Optional*. Title for the result | [optional]
**caption** | Option<**String**> | *Optional*. Caption of the GIF file to be sent, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**show_caption_above_media** | Option<**bool**> | *Optional*. Pass *True*, if the caption must be shown above the message media | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


