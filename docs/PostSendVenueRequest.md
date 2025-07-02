# PostSendVenueRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message will be sent | [optional]
**chat_id** | [**models::PostSendMessageRequestChatId**](post_sendMessage_request_chat_id.md) |  | 
**message_thread_id** | Option<**i32**> | Unique identifier for the target message thread (topic) of the forum; for forum supergroups only | [optional]
**latitude** | **f64** | Latitude of the venue | 
**longitude** | **f64** | Longitude of the venue | 
**title** | **String** | Name of the venue | 
**address** | **String** | Address of the venue | 
**foursquare_id** | Option<**String**> | Foursquare identifier of the venue | [optional]
**foursquare_type** | Option<**String**> | Foursquare type of the venue, if known. (For example, “arts\\_entertainment/default”, “arts\\_entertainment/aquarium” or “food/icecream”.) | [optional]
**google_place_id** | Option<**String**> | Google Places identifier of the venue | [optional]
**google_place_type** | Option<**String**> | Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).) | [optional]
**disable_notification** | Option<**bool**> | Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound. | [optional]
**protect_content** | Option<**bool**> | Protects the contents of the sent message from forwarding and saving | [optional]
**allow_paid_broadcast** | Option<**bool**> | Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance | [optional]
**message_effect_id** | Option<**String**> | Unique identifier of the message effect to be added to the message; for private chats only | [optional]
**reply_parameters** | Option<[**models::ReplyParameters**](ReplyParameters.md)> |  | [optional]
**reply_markup** | Option<[**models::PostSendMessageRequestReplyMarkup**](post_sendMessage_request_reply_markup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


