# BusinessConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the business connection | 
**user** | [**models::User**](User.md) |  | 
**user_chat_id** | **i32** | Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. | 
**date** | **i32** | Date the connection was established in Unix time | 
**rights** | Option<[**models::BusinessBotRights**](BusinessBotRights.md)> |  | [optional]
**is_enabled** | **bool** | True, if the connection is active | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


