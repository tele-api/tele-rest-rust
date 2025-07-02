# PostEditChatInviteLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::PostSendMessageRequestChatId**](post_sendMessage_request_chat_id.md) |  | 
**invite_link** | **String** | The invite link to edit | 
**name** | Option<**String**> | Invite link name; 0-32 characters | [optional]
**expire_date** | Option<**i32**> | Point in time (Unix timestamp) when the link will expire | [optional]
**member_limit** | Option<**i32**> | The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999 | [optional]
**creates_join_request** | Option<**bool**> | *True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\\_limit* can't be specified | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


