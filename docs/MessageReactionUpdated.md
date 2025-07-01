# MessageReactionUpdated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat** | [**models::Chat**](Chat.md) |  | 
**message_id** | **i32** | Unique identifier of the message inside the chat | 
**user** | Option<[**models::User**](User.md)> |  | [optional]
**actor_chat** | Option<[**models::Chat**](Chat.md)> |  | [optional]
**date** | **i32** | Date of the change in Unix time | 
**old_reaction** | [**Vec<models::ReactionType>**](ReactionType.md) | Previous list of reaction types that were set by the user | 
**new_reaction** | [**Vec<models::ReactionType>**](ReactionType.md) | New list of reaction types that have been set by the user | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


