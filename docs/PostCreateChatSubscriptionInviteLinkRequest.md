# PostCreateChatSubscriptionInviteLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_id** | [**models::PostCreateChatSubscriptionInviteLinkRequestChatId**](post_createChatSubscriptionInviteLink_request_chat_id.md) |  | 
**name** | Option<**String**> | Invite link name; 0-32 characters | [optional]
**subscription_period** | **i32** | The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days). | 
**subscription_price** | **i32** | The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


