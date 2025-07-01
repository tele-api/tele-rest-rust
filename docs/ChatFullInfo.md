# ChatFullInfo

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
**accent_color_id** | **i32** | Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api/#accent-colors) for more details. | 
**max_reaction_count** | **i32** | The maximum number of reactions that can be set on a message in the chat | 
**photo** | Option<[**models::ChatPhoto**](ChatPhoto.md)> |  | [optional]
**active_usernames** | Option<**Vec<String>**> | *Optional*. If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels | [optional]
**birthdate** | Option<[**models::Birthdate**](Birthdate.md)> |  | [optional]
**business_intro** | Option<[**models::BusinessIntro**](BusinessIntro.md)> |  | [optional]
**business_location** | Option<[**models::BusinessLocation**](BusinessLocation.md)> |  | [optional]
**business_opening_hours** | Option<[**models::BusinessOpeningHours**](BusinessOpeningHours.md)> |  | [optional]
**personal_chat** | Option<[**models::Chat**](Chat.md)> |  | [optional]
**available_reactions** | Option<[**Vec<models::ReactionType>**](ReactionType.md)> | *Optional*. List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api/#reactiontypeemoji) are allowed. | [optional]
**background_custom_emoji_id** | Option<**String**> | *Optional*. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background | [optional]
**profile_accent_color_id** | Option<**i32**> | *Optional*. Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api/#profile-accent-colors) for more details. | [optional]
**profile_background_custom_emoji_id** | Option<**String**> | *Optional*. Custom emoji identifier of the emoji chosen by the chat for its profile background | [optional]
**emoji_status_custom_emoji_id** | Option<**String**> | *Optional*. Custom emoji identifier of the emoji status of the chat or the other party in a private chat | [optional]
**emoji_status_expiration_date** | Option<**i32**> | *Optional*. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any | [optional]
**bio** | Option<**String**> | *Optional*. Bio of the other party in a private chat | [optional]
**has_private_forwards** | Option<**bool**> | *Optional*. *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user | [optional][default to true]
**has_restricted_voice_and_video_messages** | Option<**bool**> | *Optional*. *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat | [optional][default to true]
**join_to_send_messages** | Option<**bool**> | *Optional*. *True*, if users need to join the supergroup before they can send messages | [optional][default to true]
**join_by_request** | Option<**bool**> | *Optional*. *True*, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators | [optional][default to true]
**description** | Option<**String**> | *Optional*. Description, for groups, supergroups and channel chats | [optional]
**invite_link** | Option<**String**> | *Optional*. Primary invite link, for groups, supergroups and channel chats | [optional]
**pinned_message** | Option<[**models::Message**](Message.md)> |  | [optional]
**permissions** | Option<[**models::ChatPermissions**](ChatPermissions.md)> |  | [optional]
**accepted_gift_types** | [**models::AcceptedGiftTypes**](AcceptedGiftTypes.md) |  | 
**can_send_paid_media** | Option<**bool**> | *Optional*. *True*, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats. | [optional][default to true]
**slow_mode_delay** | Option<**i32**> | *Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds | [optional]
**unrestrict_boost_count** | Option<**i32**> | *Optional*. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions | [optional]
**message_auto_delete_time** | Option<**i32**> | *Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds | [optional]
**has_aggressive_anti_spam_enabled** | Option<**bool**> | *Optional*. *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. | [optional][default to true]
**has_hidden_members** | Option<**bool**> | *Optional*. *True*, if non-administrators can only get the list of bots and administrators in the chat | [optional][default to true]
**has_protected_content** | Option<**bool**> | *Optional*. *True*, if messages from the chat can't be forwarded to other chats | [optional][default to true]
**has_visible_history** | Option<**bool**> | *Optional*. *True*, if new chat members will have access to old messages; available only to chat administrators | [optional][default to true]
**sticker_set_name** | Option<**String**> | *Optional*. For supergroups, name of the group sticker set | [optional]
**can_set_sticker_set** | Option<**bool**> | *Optional*. *True*, if the bot can change the group sticker set | [optional][default to true]
**custom_emoji_sticker_set_name** | Option<**String**> | *Optional*. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group. | [optional]
**linked_chat_id** | Option<**i32**> | *Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. | [optional]
**location** | Option<[**models::ChatLocation**](ChatLocation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


