# InlineQueryResultArticle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *article* | [default to article]
**id** | **String** | Unique identifier for this result, 1-64 Bytes | 
**title** | **String** | Title of the result | 
**input_message_content** | [**models::InputMessageContent**](InputMessageContent.md) |  | 
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**url** | Option<**String**> | *Optional*. URL of the result | [optional]
**description** | Option<**String**> | *Optional*. Short description of the result | [optional]
**thumbnail_url** | Option<**String**> | *Optional*. Url of the thumbnail for the result | [optional]
**thumbnail_width** | Option<**i32**> | *Optional*. Thumbnail width | [optional]
**thumbnail_height** | Option<**i32**> | *Optional*. Thumbnail height | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


