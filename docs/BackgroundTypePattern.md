# BackgroundTypePattern

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the background, always “pattern” | [default to pattern]
**document** | [**models::Document**](Document.md) |  | 
**fill** | [**models::BackgroundFill**](BackgroundFill.md) |  | 
**intensity** | **i32** | Intensity of the pattern when it is shown above the filled background; 0-100 | 
**is_inverted** | Option<**bool**> | *Optional*. *True*, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only | [optional][default to true]
**is_moving** | Option<**bool**> | *Optional*. *True*, if the background moves slightly when the device is tilted | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


