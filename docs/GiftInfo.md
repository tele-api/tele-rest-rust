# GiftInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gift** | [**models::Gift**](Gift.md) |  | 
**owned_gift_id** | Option<**String**> | *Optional*. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts | [optional]
**convert_star_count** | Option<**i32**> | *Optional*. Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible | [optional]
**prepaid_upgrade_star_count** | Option<**i32**> | *Optional*. Number of Telegram Stars that were prepaid by the sender for the ability to upgrade the gift | [optional]
**can_be_upgraded** | Option<**bool**> | *Optional*. True, if the gift can be upgraded to a unique gift | [optional][default to true]
**text** | Option<**String**> | *Optional*. Text of the message that was added to the gift | [optional]
**entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. Special entities that appear in the text | [optional]
**is_private** | Option<**bool**> | *Optional*. True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


