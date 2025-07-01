# UniqueGiftInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gift** | [**models::UniqueGift**](UniqueGift.md) |  | 
**origin** | **String** | Origin of the gift. Currently, either “upgrade” or “transfer” | 
**owned_gift_id** | Option<**String**> | *Optional*. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts | [optional]
**transfer_star_count** | Option<**i32**> | *Optional*. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


