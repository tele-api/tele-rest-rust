# BackgroundType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the background, always “chat\\_theme” | [default to chat_theme]
**fill** | [**models::BackgroundFill**](BackgroundFill.md) |  | 
**dark_theme_dimming** | **i32** | Dimming of the background in dark themes, as a percentage; 0-100 | 
**document** | [**models::Document**](Document.md) |  | 
**is_blurred** | Option<**bool**> | *Optional*. *True*, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12 | [optional][default to true]
**is_moving** | Option<**bool**> | *Optional*. *True*, if the background moves slightly when the device is tilted | [optional][default to true]
**intensity** | **i32** | Intensity of the pattern when it is shown above the filled background; 0-100 | 
**is_inverted** | Option<**bool**> | *Optional*. *True*, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only | [optional][default to true]
**theme_name** | **String** | Name of the chat theme, which is usually an emoji | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


