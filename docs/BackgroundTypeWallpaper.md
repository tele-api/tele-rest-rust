# BackgroundTypeWallpaper

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the background, always “wallpaper” | [default to wallpaper]
**document** | [**models::Document**](Document.md) |  | 
**dark_theme_dimming** | **i32** | Dimming of the background in dark themes, as a percentage; 0-100 | 
**is_blurred** | Option<**bool**> | *Optional*. *True*, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12 | [optional][default to true]
**is_moving** | Option<**bool**> | *Optional*. *True*, if the background moves slightly when the device is tilted | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


