# TransactionPartnerUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the transaction partner, always “user” | [default to user]
**transaction_type** | **String** | Type of the transaction, currently one of “invoice\\_payment” for payments via invoices, “paid\\_media\\_payment” for payments for paid media, “gift\\_purchase” for gifts sent by the bot, “premium\\_purchase” for Telegram Premium subscriptions gifted by the bot, “business\\_account\\_transfer” for direct transfers from managed business accounts | 
**user** | [**models::User**](User.md) |  | 
**affiliate** | Option<[**models::AffiliateInfo**](AffiliateInfo.md)> |  | [optional]
**invoice_payload** | Option<**String**> | *Optional*. Bot-specified invoice payload. Can be available only for “invoice\\_payment” transactions. | [optional]
**subscription_period** | Option<**i32**> | *Optional*. The duration of the paid subscription. Can be available only for “invoice\\_payment” transactions. | [optional]
**paid_media** | Option<[**Vec<models::PaidMedia>**](PaidMedia.md)> | *Optional*. Information about the paid media bought by the user; for “paid\\_media\\_payment” transactions only | [optional]
**paid_media_payload** | Option<**String**> | *Optional*. Bot-specified paid media payload. Can be available only for “paid\\_media\\_payment” transactions. | [optional]
**gift** | Option<[**models::Gift**](Gift.md)> |  | [optional]
**premium_subscription_duration** | Option<**i32**> | *Optional*. Number of months the gifted Telegram Premium subscription will be active for; for “premium\\_purchase” transactions only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


