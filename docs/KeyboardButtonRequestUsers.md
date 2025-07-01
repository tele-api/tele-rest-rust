# KeyboardButtonRequestUsers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **i32** | Signed 32-bit identifier of the request that will be received back in the [UsersShared](https://core.telegram.org/bots/api/#usersshared) object. Must be unique within the message | 
**user_is_bot** | Option<**bool**> | *Optional*. Pass *True* to request bots, pass *False* to request regular users. If not specified, no additional restrictions are applied. | [optional]
**user_is_premium** | Option<**bool**> | *Optional*. Pass *True* to request premium users, pass *False* to request non-premium users. If not specified, no additional restrictions are applied. | [optional]
**max_quantity** | Option<**i32**> | *Optional*. The maximum number of users to be selected; 1-10. Defaults to 1. | [optional][default to 1]
**request_name** | Option<**bool**> | *Optional*. Pass *True* to request the users' first and last names | [optional]
**request_username** | Option<**bool**> | *Optional*. Pass *True* to request the users' usernames | [optional]
**request_photo** | Option<**bool**> | *Optional*. Pass *True* to request the users' photos | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


