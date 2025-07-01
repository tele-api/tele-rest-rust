# PollAnswer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**poll_id** | **String** | Unique poll identifier | 
**voter_chat** | Option<[**models::Chat**](Chat.md)> |  | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**option_ids** | **Vec<i32>** | 0-based identifiers of chosen answer options. May be empty if the vote was retracted. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


