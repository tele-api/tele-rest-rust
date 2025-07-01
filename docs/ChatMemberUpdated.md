# ChatMemberUpdated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat** | [**models::Chat**](Chat.md) |  | 
**from** | [**models::User**](User.md) |  | 
**date** | **i32** | Date the change was done in Unix time | 
**old_chat_member** | [**models::ChatMember**](ChatMember.md) |  | 
**new_chat_member** | [**models::ChatMember**](ChatMember.md) |  | 
**invite_link** | Option<[**models::ChatInviteLink**](ChatInviteLink.md)> |  | [optional]
**via_join_request** | Option<**bool**> | *Optional*. True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator | [optional]
**via_chat_folder_invite_link** | Option<**bool**> | *Optional*. True, if the user joined the chat via a chat folder invite link | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


