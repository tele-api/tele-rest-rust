# InlineQueryResultVenue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *venue* | [default to venue]
**id** | **String** | Unique identifier for this result, 1-64 Bytes | 
**latitude** | **f64** | Latitude of the venue location in degrees | 
**longitude** | **f64** | Longitude of the venue location in degrees | 
**title** | **String** | Title of the venue | 
**address** | **String** | Address of the venue | 
**foursquare_id** | Option<**String**> | *Optional*. Foursquare identifier of the venue if known | [optional]
**foursquare_type** | Option<**String**> | *Optional*. Foursquare type of the venue, if known. (For example, “arts\\_entertainment/default”, “arts\\_entertainment/aquarium” or “food/icecream”.) | [optional]
**google_place_id** | Option<**String**> | *Optional*. Google Places identifier of the venue | [optional]
**google_place_type** | Option<**String**> | *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).) | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]
**thumbnail_url** | Option<**String**> | *Optional*. Url of the thumbnail for the result | [optional]
**thumbnail_width** | Option<**i32**> | *Optional*. Thumbnail width | [optional]
**thumbnail_height** | Option<**i32**> | *Optional*. Thumbnail height | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


