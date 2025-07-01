# MessageOriginChat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the message origin, always “chat” | [default to chat]
**date** | **i32** | Date the message was sent originally in Unix time | 
**sender_chat** | [**models::Chat**](Chat.md) |  | 
**author_signature** | Option<**String**> | *Optional*. For messages originally sent by an anonymous chat administrator, original message author signature | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


