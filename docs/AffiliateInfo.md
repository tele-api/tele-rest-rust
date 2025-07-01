# AffiliateInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affiliate_user** | Option<[**models::User**](User.md)> |  | [optional]
**affiliate_chat** | Option<[**models::Chat**](Chat.md)> |  | [optional]
**commission_per_mille** | **i32** | The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users | 
**amount** | **i32** | Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds | 
**nanostar_amount** | Option<**i32**> | *Optional*. The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


