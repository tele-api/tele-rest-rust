# GiftPremiumSubscriptionPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **i32** | Unique identifier of the target user who will receive a Telegram Premium subscription | 
**month_count** | **i32** | Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12 | 
**star_count** | **i32** | Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months | 
**text** | Option<**String**> | Text that will be shown along with the service message about the subscription; 0-128 characters | [optional]
**text_parse_mode** | Option<**String**> | Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\\_emoji” are ignored. | [optional]
**text_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\\_parse\\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\\_emoji” are ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


