# PostEditMessageTextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message to be edited was sent | [optional]
**chat_id** | Option<[**models::PostEditMessageTextRequestChatId**](post_editMessageText_request_chat_id.md)> |  | [optional]
**message_id** | Option<**i32**> | Required if *inline\\_message\\_id* is not specified. Identifier of the message to edit | [optional]
**inline_message_id** | Option<**String**> | Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message | [optional]
**text** | **String** | New text of the message, 1-4096 characters after entities parsing | 
**parse_mode** | Option<**String**> | Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\\_mode* | [optional]
**link_preview_options** | Option<[**models::LinkPreviewOptions**](LinkPreviewOptions.md)> |  | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


