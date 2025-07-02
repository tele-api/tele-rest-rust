# PostCopyMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::PostSendMessageRequestChatId**](post_sendMessage_request_chat_id.md) |  | 
**message_thread_id** | Option<**i32**> | Unique identifier for the target message thread (topic) of the forum; for forum supergroups only | [optional]
**from_chat_id** | [**models::PostForwardMessageRequestFromChatId**](post_forwardMessage_request_from_chat_id.md) |  | 
**message_id** | **i32** | Message identifier in the chat specified in *from\\_chat\\_id* | 
**video_start_timestamp** | Option<**i32**> | New start timestamp for the copied video in the message | [optional]
**caption** | Option<**String**> | New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept | [optional]
**parse_mode** | Option<**String**> | Mode for parsing entities in the new caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of *parse\\_mode* | [optional]
**show_caption_above_media** | Option<**bool**> | Pass *True*, if the caption must be shown above the message media. Ignored if a new caption isn't specified. | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**protect_content** | Option<**bool**> | Protects the contents of the sent message from forwarding and saving | [optional]
**allow_paid_broadcast** | Option<**bool**> | Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance | [optional]
**reply_parameters** | Option<[**models::ReplyParameters**](ReplyParameters.md)> |  | [optional]
**reply_markup** | Option<[**models::PostSendMessageRequestReplyMarkup**](post_sendMessage_request_reply_markup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


