# PostEditMessageLiveLocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | Option<**String**> | Unique identifier of the business connection on behalf of which the message to be edited was sent | [optional]
**chat_id** | Option<[**models::PostEditMessageTextRequestChatId**](post_editMessageText_request_chat_id.md)> |  | [optional]
**message_id** | Option<**i32**> | Required if *inline\\_message\\_id* is not specified. Identifier of the message to edit | [optional]
**inline_message_id** | Option<**String**> | Required if *chat\\_id* and *message\\_id* are not specified. Identifier of the inline message | [optional]
**latitude** | **f64** | Latitude of new location | 
**longitude** | **f64** | Longitude of new location | 
**live_period** | Option<**i32**> | New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current *live\\_period* by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then *live\\_period* remains unchanged | [optional]
**horizontal_accuracy** | Option<**f64**> | The radius of uncertainty for the location, measured in meters; 0-1500 | [optional]
**heading** | Option<**i32**> | Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified. | [optional]
**proximity_alert_radius** | Option<**i32**> | The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified. | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


