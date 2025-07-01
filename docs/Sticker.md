# Sticker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | **String** | Identifier for this file, which can be used to download or reuse the file | 
**file_unique_id** | **String** | Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file. | 
**r#type** | **String** | Type of the sticker, currently one of “regular”, “mask”, “custom\\_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is\\_animated* and *is\\_video*. | 
**width** | **i32** | Sticker width | 
**height** | **i32** | Sticker height | 
**is_animated** | **bool** | *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers) | 
**is_video** | **bool** | *True*, if the sticker is a [video sticker](https://telegram.org/blog/video-stickers-better-reactions) | 
**thumbnail** | Option<[**models::PhotoSize**](PhotoSize.md)> |  | [optional]
**emoji** | Option<**String**> | *Optional*. Emoji associated with the sticker | [optional]
**set_name** | Option<**String**> | *Optional*. Name of the sticker set to which the sticker belongs | [optional]
**premium_animation** | Option<[**models::File**](File.md)> |  | [optional]
**mask_position** | Option<[**models::MaskPosition**](MaskPosition.md)> |  | [optional]
**custom_emoji_id** | Option<**String**> | *Optional*. For custom emoji stickers, unique identifier of the custom emoji | [optional]
**needs_repainting** | Option<**bool**> | *Optional*. *True*, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places | [optional][default to true]
**file_size** | Option<**i32**> | *Optional*. File size in bytes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


