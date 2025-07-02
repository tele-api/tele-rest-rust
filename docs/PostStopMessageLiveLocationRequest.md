# PostStopMessageLiveLocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message to be edited was sent | [optional]
**chat_id** | Option<[**models::PostEditMessageTextRequestChatId**](post_editMessageText_request_chat_id.md)> |  | [optional]
**message_id** | Option<**i32**> | Required if *inline\\_message\\_id* is not specified. Identifier of the message with live location to stop | [optional]
**inline_message_id** | Option<**String**> | Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


