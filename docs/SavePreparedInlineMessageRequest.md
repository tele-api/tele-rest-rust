# SavePreparedInlineMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **i32** | Unique identifier of the target user that can use the prepared message | 
**result** | [**models::InlineQueryResult**](InlineQueryResult.md) |  | 
**allow_user_chats** | Option<**bool**> | Pass *True* if the message can be sent to private chats with users | [optional]
**allow_bot_chats** | Option<**bool**> | Pass *True* if the message can be sent to private chats with bots | [optional]
**allow_group_chats** | Option<**bool**> | Pass *True* if the message can be sent to group and supergroup chats | [optional]
**allow_channel_chats** | Option<**bool**> | Pass *True* if the message can be sent to channel chats | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


