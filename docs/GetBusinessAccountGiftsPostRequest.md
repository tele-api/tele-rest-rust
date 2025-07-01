# GetBusinessAccountGiftsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | **String** | Unique identifier of the business connection | 
**exclude_unsaved** | Option<**bool**> | Pass True to exclude gifts that aren't saved to the account's profile page | [optional]
**exclude_saved** | Option<**bool**> | Pass True to exclude gifts that are saved to the account's profile page | [optional]
**exclude_unlimited** | Option<**bool**> | Pass True to exclude gifts that can be purchased an unlimited number of times | [optional]
**exclude_limited** | Option<**bool**> | Pass True to exclude gifts that can be purchased a limited number of times | [optional]
**exclude_unique** | Option<**bool**> | Pass True to exclude unique gifts | [optional]
**sort_by_price** | Option<**bool**> | Pass True to sort results by gift price instead of send date. Sorting is applied before pagination. | [optional]
**offset** | Option<**String**> | Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results | [optional]
**limit** | Option<**i32**> | The maximum number of gifts to be returned; 1-100. Defaults to 100 | [optional][default to 100]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


