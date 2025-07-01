# SharedUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **i32** | Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means. | 
**first_name** | Option<**String**> | *Optional*. First name of the user, if the name was requested by the bot | [optional]
**last_name** | Option<**String**> | *Optional*. Last name of the user, if the name was requested by the bot | [optional]
**username** | Option<**String**> | *Optional*. Username of the user, if the username was requested by the bot | [optional]
**photo** | Option<[**Vec<models::PhotoSize>**](PhotoSize.md)> | *Optional*. Available sizes of the chat photo, if the photo was requested by the bot | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


