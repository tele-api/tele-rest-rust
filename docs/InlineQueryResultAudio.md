# InlineQueryResultAudio

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *audio* | [default to audio]
**id** | **String** | Unique identifier for this result, 1-64 bytes | 
**audio_url** | **String** | A valid URL for the audio file | 
**title** | **String** | Title | 
**caption** | Option<**String**> | *Optional*. Caption, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**performer** | Option<**String**> | *Optional*. Performer | [optional]
**audio_duration** | Option<**i32**> | *Optional*. Audio duration in seconds | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


