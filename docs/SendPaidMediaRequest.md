# SendPaidMediaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message will be sent | [optional]
**chat_id** | [**models::SendPaidMediaRequestChatId**](sendPaidMediaRequest_chat_id.md) |  | 
**star_count** | **i32** | The number of Telegram Stars that must be paid to buy access to the media; 1-10000 | 
**media** | [**Vec<models::InputPaidMedia>**](InputPaidMedia.md) | A JSON-serialized array describing the media to be sent; up to 10 items | 
**payload** | Option<**String**> | Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes. | [optional]
**caption** | Option<**String**> | Media caption, 0-1024 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | Mode for parsing entities in the media caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**show_caption_above_media** | Option<**bool**> | Pass *True*, if the caption must be shown above the message media | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**protect_content** | Option<**bool**> | Protects the contents of the sent message from forwarding and saving | [optional]
**allow_paid_broadcast** | Option<**bool**> | Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance | [optional]
**reply_parameters** | Option<[**models::ReplyParameters**](ReplyParameters.md)> |  | [optional]
**reply_markup** | Option<[**models::SendMessageRequestReplyMarkup**](sendMessageRequest_reply_markup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


