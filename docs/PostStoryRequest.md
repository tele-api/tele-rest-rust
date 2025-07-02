# PostStoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_connection_id** | **String** | Unique identifier of the business connection | 
**content** | [**models::InputStoryContent**](InputStoryContent.md) |  | 
**active_period** | **i32** | Period after which the story is moved to the archive, in seconds; must be one of `6 * 3600`, `12 * 3600`, `86400`, or `2 * 86400` | 
**caption** | Option<**String**> | Caption of the story, 0-2048 characters after entities parsing | [optional]
**parse_mode** | Option<**String**> | Mode for parsing entities in the story caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**caption_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\\_mode* | [optional]
**areas** | Option<[**Vec<models::StoryArea>**](StoryArea.md)> | A JSON-serialized list of clickable areas to be shown on the story | [optional]
**post_to_chat_page** | Option<**bool**> | Pass *True* to keep the story accessible after it expires | [optional]
**protect_content** | Option<**bool**> | Pass *True* if the content of the story must be protected from forwarding and screenshotting | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


