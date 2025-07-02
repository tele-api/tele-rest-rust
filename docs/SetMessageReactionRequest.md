# SetMessageReactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::SendMessageRequestChatId**](sendMessageRequest_chat_id.md) |  | 
**message_id** | **i32** | Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead. | 
**reaction** | Option<[**Vec<models::ReactionType>**](ReactionType.md)> | A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots. | [optional]
**is_big** | Option<**bool**> | Pass *True* to set the reaction with a big animation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


