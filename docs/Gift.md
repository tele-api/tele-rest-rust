# Gift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the gift | 
**sticker** | [**models::Sticker**](Sticker.md) |  | 
**star_count** | **i32** | The number of Telegram Stars that must be paid to send the sticker | 
**upgrade_star_count** | Option<**i32**> | *Optional*. The number of Telegram Stars that must be paid to upgrade the gift to a unique one | [optional]
**total_count** | Option<**i32**> | *Optional*. The total number of the gifts of this type that can be sent; for limited gifts only | [optional]
**remaining_count** | Option<**i32**> | *Optional*. The number of remaining gifts of this type that can be sent; for limited gifts only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


