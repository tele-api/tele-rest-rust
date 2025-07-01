# InlineQueryResultVideo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *video* | [default to video]
**id** | **String** | Unique identifier for this result, 1-64 bytes | 
**video_url** | **String** | A valid URL for the embedded video player or video file | 
**mime_type** | **String** | MIME type of the content of the video URL, “text/html” or “video/mp4” | 
**thumbnail_url** | **String** | URL of the thumbnail (JPEG only) for the video | 
**title** | **String** | Title for the result | 
**caption** | Option<**String**> | *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**show_caption_above_media** | Option<**bool**> | *Optional*. Pass *True*, if the caption must be shown above the message media | [optional]
**video_width** | Option<**i32**> | *Optional*. Video width | [optional]
**video_height** | Option<**i32**> | *Optional*. Video height | [optional]
**video_duration** | Option<**i32**> | *Optional*. Video duration in seconds | [optional]
**description** | Option<**String**> | *Optional*. Short description of the result | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


