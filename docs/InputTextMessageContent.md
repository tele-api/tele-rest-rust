# InputTextMessageContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_text** | **String** | Text of the message to be sent, 1-4096 characters | 
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in message text, which can be specified instead of *parse\\_mode* | [optional]
**link_preview_options** | Option<[**models::LinkPreviewOptions**](LinkPreviewOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


