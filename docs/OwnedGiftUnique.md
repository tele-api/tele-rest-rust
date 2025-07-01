# OwnedGiftUnique

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the gift, always “unique” | [default to unique]
**gift** | [**models::UniqueGift**](UniqueGift.md) |  | 
**owned_gift_id** | Option<**String**> | *Optional*. Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only | [optional]
**sender_user** | Option<[**models::User**](User.md)> |  | [optional]
**send_date** | **i32** | Date the gift was sent in Unix time | 
**is_saved** | Option<**bool**> | *Optional*. True, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only | [optional][default to true]
**can_be_transferred** | Option<**bool**> | *Optional*. True, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only | [optional][default to true]
**transfer_star_count** | Option<**i32**> | *Optional*. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


