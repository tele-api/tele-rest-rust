# Chat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier. | 
**r#type** | **String** | Type of the chat, can be either “private”, “group”, “supergroup” or “channel” | 
**title** | Option<**String**> | *Optional*. Title, for supergroups, channels and group chats | [optional]
**username** | Option<**String**> | *Optional*. Username, for private chats, supergroups and channels if available | [optional]
**first_name** | Option<**String**> | *Optional*. First name of the other party in a private chat | [optional]
**last_name** | Option<**String**> | *Optional*. Last name of the other party in a private chat | [optional]
**is_forum** | Option<**bool**> | *Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled) | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


