# PromoteChatMemberPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::SendMessagePostRequestChatId**](_sendMessage_post_request_chat_id.md) |  | 
**user_id** | **i32** | Unique identifier of the target user | 
**is_anonymous** | Option<**bool**> | Pass *True* if the administrator's presence in the chat is hidden | [optional]
**can_manage_chat** | Option<**bool**> | Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege. | [optional]
**can_delete_messages** | Option<**bool**> | Pass *True* if the administrator can delete messages of other users | [optional]
**can_manage_video_chats** | Option<**bool**> | Pass *True* if the administrator can manage video chats | [optional]
**can_restrict_members** | Option<**bool**> | Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics | [optional]
**can_promote_members** | Option<**bool**> | Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him) | [optional]
**can_change_info** | Option<**bool**> | Pass *True* if the administrator can change chat title, photo and other settings | [optional]
**can_invite_users** | Option<**bool**> | Pass *True* if the administrator can invite new users to the chat | [optional]
**can_post_stories** | Option<**bool**> | Pass *True* if the administrator can post stories to the chat | [optional]
**can_edit_stories** | Option<**bool**> | Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive | [optional]
**can_delete_stories** | Option<**bool**> | Pass *True* if the administrator can delete stories posted by other users | [optional]
**can_post_messages** | Option<**bool**> | Pass *True* if the administrator can post messages in the channel, or access channel statistics; for channels only | [optional]
**can_edit_messages** | Option<**bool**> | Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only | [optional]
**can_pin_messages** | Option<**bool**> | Pass *True* if the administrator can pin messages; for supergroups only | [optional]
**can_manage_topics** | Option<**bool**> | Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


