# PostSendMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message will be sent | [optional]
**chat_id** | [**models::PostSendMessageRequestChatId**](post_sendMessage_request_chat_id.md) |  | 
**message_thread_id** | Option<**i32**> | Unique identifier for the target message thread (topic) of the forum; for forum supergroups only | [optional]
**text** | **String** | Text of the message to be sent, 1-4096 characters after entities parsing | 
**parse_mode** | Option<**String**> | Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\\_mode* | [optional]
**link_preview_options** | Option<[**models::LinkPreviewOptions**](LinkPreviewOptions.md)> |  | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**protect_content** | Option<**bool**> | Protects the contents of the sent message from forwarding and saving | [optional]
**allow_paid_broadcast** | Option<**bool**> | Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance | [optional]
**message_effect_id** | Option<**String**> | Unique identifier of the message effect to be added to the message; for private chats only | [optional]
**reply_parameters** | Option<[**models::ReplyParameters**](ReplyParameters.md)> |  | [optional]
**reply_markup** | Option<[**models::PostSendMessageRequestReplyMarkup**](post_sendMessage_request_reply_markup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


