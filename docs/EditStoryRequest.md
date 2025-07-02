# EditStoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | **String** | Unique identifier of the business connection | 
**story_id** | **i32** | Unique identifier of the story to edit | 
**content** | [**models::InputStoryContent**](InputStoryContent.md) |  | 
**caption** | Option<**String**> | Caption of the story, 0-2048 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | Mode for parsing entities in the story caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**areas** | Option<[**Vec<models::StoryArea>**](StoryArea.md)> | A JSON-serialized list of clickable areas to be shown on the story | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


