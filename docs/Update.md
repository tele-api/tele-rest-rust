# Update

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**update_id** | **i32** | The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially. | 
**message** | Option<[**models::Message**](Message.md)> |  | [optional]
**edited_message** | Option<[**models::Message**](Message.md)> |  | [optional]
**channel_post** | Option<[**models::Message**](Message.md)> |  | [optional]
**edited_channel_post** | Option<[**models::Message**](Message.md)> |  | [optional]
**business_connection** | Option<[**models::BusinessConnection**](BusinessConnection.md)> |  | [optional]
**business_message** | Option<[**models::Message**](Message.md)> |  | [optional]
**edited_business_message** | Option<[**models::Message**](Message.md)> |  | [optional]
**deleted_business_messages** | Option<[**models::BusinessMessagesDeleted**](BusinessMessagesDeleted.md)> |  | [optional]
**message_reaction** | Option<[**models::MessageReactionUpdated**](MessageReactionUpdated.md)> |  | [optional]
**message_reaction_count** | Option<[**models::MessageReactionCountUpdated**](MessageReactionCountUpdated.md)> |  | [optional]
**inline_query** | Option<[**models::InlineQuery**](InlineQuery.md)> |  | [optional]
**chosen_inline_result** | Option<[**models::ChosenInlineResult**](ChosenInlineResult.md)> |  | [optional]
**callback_query** | Option<[**models::CallbackQuery**](CallbackQuery.md)> |  | [optional]
**shipping_query** | Option<[**models::ShippingQuery**](ShippingQuery.md)> |  | [optional]
**pre_checkout_query** | Option<[**models::PreCheckoutQuery**](PreCheckoutQuery.md)> |  | [optional]
**purchased_paid_media** | Option<[**models::PaidMediaPurchased**](PaidMediaPurchased.md)> |  | [optional]
**poll** | Option<[**models::Poll**](Poll.md)> |  | [optional]
**poll_answer** | Option<[**models::PollAnswer**](PollAnswer.md)> |  | [optional]
**my_chat_member** | Option<[**models::ChatMemberUpdated**](ChatMemberUpdated.md)> |  | [optional]
**chat_member** | Option<[**models::ChatMemberUpdated**](ChatMemberUpdated.md)> |  | [optional]
**chat_join_request** | Option<[**models::ChatJoinRequest**](ChatJoinRequest.md)> |  | [optional]
**chat_boost** | Option<[**models::ChatBoostUpdated**](ChatBoostUpdated.md)> |  | [optional]
**removed_chat_boost** | Option<[**models::ChatBoostRemoved**](ChatBoostRemoved.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


