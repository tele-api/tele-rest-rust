# PostForwardMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::PostSendMessageRequestChatId**](post_sendMessage_request_chat_id.md) |  | 
**message_thread_id** | Option<**i32**> | Unique identifier for the target message thread (topic) of the forum; for forum supergroups only | [optional]
**from_chat_id** | [**models::PostForwardMessageRequestFromChatId**](post_forwardMessage_request_from_chat_id.md) |  | 
**video_start_timestamp** | Option<**i32**> | New start timestamp for the forwarded video in the message | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**protect_content** | Option<**bool**> | Protects the contents of the forwarded message from forwarding and saving | [optional]
**message_id** | **i32** | Message identifier in the chat specified in *from\\_chat\\_id* | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


