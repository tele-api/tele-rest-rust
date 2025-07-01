# CopyMessagesPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::SendMessagePostRequestChatId**](_sendMessage_post_request_chat_id.md) |  | 
**message_thread_id** | Option<**i32**> | Unique identifier for the target message thread (topic) of the forum; for forum supergroups only | [optional]
**from_chat_id** | [**models::ForwardMessagesPostRequestFromChatId**](_forwardMessages_post_request_from_chat_id.md) |  | 
**message_ids** | **Vec<i32>** | A JSON-serialized list of 1-100 identifiers of messages in the chat *from\\_chat\\_id* to copy. The identifiers must be specified in a strictly increasing order. | 
**disable_notification** | Option<**bool**> | Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**protect_content** | Option<**bool**> | Protects the contents of the sent messages from forwarding and saving | [optional]
**remove_caption** | Option<**bool**> | Pass *True* to copy the messages without their captions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


