# ChatBoostSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | **String** | Source of the boost, always “giveaway” | [default to giveaway]
**user** | [**models::User**](User.md) |  | 
**giveaway_message_id** | **i32** | Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet. | 
**prize_star_count** | Option<**i32**> | *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only | [optional]
**is_unclaimed** | Option<**bool**> | *Optional*. True, if the giveaway was completed, but there was no user to win the prize | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


