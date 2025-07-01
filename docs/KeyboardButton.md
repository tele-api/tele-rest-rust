# KeyboardButton

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed | 
**request_users** | Option<[**models::KeyboardButtonRequestUsers**](KeyboardButtonRequestUsers.md)> |  | [optional]
**request_chat** | Option<[**models::KeyboardButtonRequestChat**](KeyboardButtonRequestChat.md)> |  | [optional]
**request_contact** | Option<**bool**> | *Optional*. If *True*, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only. | [optional]
**request_location** | Option<**bool**> | *Optional*. If *True*, the user's current location will be sent when the button is pressed. Available in private chats only. | [optional]
**request_poll** | Option<[**models::KeyboardButtonPollType**](KeyboardButtonPollType.md)> |  | [optional]
**web_app** | Option<[**models::WebAppInfo**](WebAppInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


