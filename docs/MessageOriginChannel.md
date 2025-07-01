# MessageOriginChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the message origin, always “channel” | [default to channel]
**date** | **i32** | Date the message was sent originally in Unix time | 
**chat** | [**models::Chat**](Chat.md) |  | 
**message_id** | **i32** | Unique message identifier inside the chat | 
**author_signature** | Option<**String**> | *Optional*. Signature of the original post author | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


