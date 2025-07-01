# DeleteBusinessMessagesPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | **String** | Unique identifier of the business connection on behalf of which to delete the messages | 
**message_ids** | **Vec<i32>** | A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [deleteMessage](https://core.telegram.org/bots/api/#deletemessage) for limitations on which messages can be deleted | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


