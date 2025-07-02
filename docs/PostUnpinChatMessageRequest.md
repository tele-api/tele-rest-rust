# PostUnpinChatMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message will be unpinned | [optional]
**chat_id** | [**models::PostSendMessageRequestChatId**](post_sendMessage_request_chat_id.md) |  | 
**message_id** | Option<**i32**> | Identifier of the message to unpin. Required if *business\\_connection\\_id* is specified. If not specified, the most recent pinned message (by sending date) will be unpinned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


