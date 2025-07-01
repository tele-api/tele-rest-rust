# UpgradeGiftPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | **String** | Unique identifier of the business connection | 
**owned_gift_id** | **String** | Unique identifier of the regular gift that should be upgraded to a unique one | 
**keep_original_details** | Option<**bool**> | Pass True to keep the original gift text, sender and receiver in the upgraded gift | [optional]
**star_count** | Option<**i32**> | The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count > 0`, then pass 0, otherwise, the *can\\_transfer\\_stars* business bot right is required and `gift.upgrade_star_count` must be passed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


