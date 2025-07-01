# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. | 
**is_bot** | **bool** | *True*, if this user is a bot | 
**first_name** | **String** | User's or bot's first name | 
**last_name** | Option<**String**> | *Optional*. User's or bot's last name | [optional]
**username** | Option<**String**> | *Optional*. User's or bot's username | [optional]
**language_code** | Option<**String**> | *Optional*. [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language | [optional]
**is_premium** | Option<**bool**> | *Optional*. *True*, if this user is a Telegram Premium user | [optional][default to true]
**added_to_attachment_menu** | Option<**bool**> | *Optional*. *True*, if this user added the bot to the attachment menu | [optional][default to true]
**can_join_groups** | Option<**bool**> | *Optional*. *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api/#getme). | [optional]
**can_read_all_group_messages** | Option<**bool**> | *Optional*. *True*, if [privacy mode](https://core.telegram.org/bots/features#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api/#getme). | [optional]
**supports_inline_queries** | Option<**bool**> | *Optional*. *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api/#getme). | [optional]
**can_connect_to_business** | Option<**bool**> | *Optional*. *True*, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [getMe](https://core.telegram.org/bots/api/#getme). | [optional]
**has_main_web_app** | Option<**bool**> | *Optional*. *True*, if the bot has a main Web App. Returned only in [getMe](https://core.telegram.org/bots/api/#getme). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


