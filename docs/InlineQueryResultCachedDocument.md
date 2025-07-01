# InlineQueryResultCachedDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *document* | [default to document]
**id** | **String** | Unique identifier for this result, 1-64 bytes | 
**title** | **String** | Title for the result | 
**document_file_id** | **String** | A valid file identifier for the file | 
**description** | Option<**String**> | *Optional*. Short description of the result | [optional]
**caption** | Option<**String**> | *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


