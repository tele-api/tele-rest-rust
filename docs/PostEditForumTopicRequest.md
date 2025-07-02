# PostEditForumTopicRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::PostRestrictChatMemberRequestChatId**](post_restrictChatMember_request_chat_id.md) |  | 
**message_thread_id** | **i32** | Unique identifier for the target message thread of the forum topic | 
**name** | Option<**String**> | New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept | [optional]
**icon_custom_emoji_id** | Option<**String**> | New unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


