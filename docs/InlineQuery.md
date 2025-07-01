# InlineQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for this query | 
**from** | [**models::User**](User.md) |  | 
**query** | **String** | Text of the query (up to 256 characters) | 
**offset** | **String** | Offset of the results to be returned, can be controlled by the bot | 
**chat_type** | Option<**String**> | *Optional*. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat | [optional]
**location** | Option<[**models::Location**](Location.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


