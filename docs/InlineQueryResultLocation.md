# InlineQueryResultLocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *location* | [default to location]
**id** | **String** | Unique identifier for this result, 1-64 Bytes | 
**latitude** | **f64** | Location latitude in degrees | 
**longitude** | **f64** | Location longitude in degrees | 
**title** | **String** | Location title | 
**horizontal_accuracy** | Option<**f64**> | *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500 | [optional]
**live_period** | Option<**i32**> | *Optional*. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely. | [optional]
**heading** | Option<**i32**> | *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified. | [optional]
**proximity_alert_radius** | Option<**i32**> | *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified. | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]
**thumbnail_url** | Option<**String**> | *Optional*. Url of the thumbnail for the result | [optional]
**thumbnail_width** | Option<**i32**> | *Optional*. Thumbnail width | [optional]
**thumbnail_height** | Option<**i32**> | *Optional*. Thumbnail height | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


