# PostSendGiftRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | Required if *chat\\_id* is not specified. Unique identifier of the target user who will receive the gift. | [optional]
**chat_id** | Option<[**models::PostSendGiftRequestChatId**](post_sendGift_request_chat_id.md)> |  | [optional]
**gift_id** | **String** | Identifier of the gift | 
**pay_for_upgrade** | Option<**bool**> | Pass *True* to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver | [optional]
**text** | Option<**String**> | Text that will be shown along with the gift; 0-128 characters | [optional]
**text_parse_mode** | Option<**String**> | Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\\_emoji” are ignored. | [optional]
**text_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\\_parse\\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\\_emoji” are ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


