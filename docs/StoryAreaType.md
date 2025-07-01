# StoryAreaType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the area, always “unique\\_gift” | [default to unique_gift]
**latitude** | **f64** | Location latitude in degrees | 
**longitude** | **f64** | Location longitude in degrees | 
**address** | Option<[**models::LocationAddress**](LocationAddress.md)> |  | [optional]
**reaction_type** | [**models::ReactionType**](ReactionType.md) |  | 
**is_dark** | Option<**bool**> | *Optional*. Pass *True* if the reaction area has a dark background | [optional]
**is_flipped** | Option<**bool**> | *Optional*. Pass *True* if reaction area corner is flipped | [optional]
**url** | **String** | HTTP or tg:// URL to be opened when the area is clicked | 
**temperature** | **f64** | Temperature, in degree Celsius | 
**emoji** | **String** | Emoji representing the weather | 
**background_color** | **i32** | A color of the area background in the ARGB format | 
**name** | **String** | Unique name of the gift | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


