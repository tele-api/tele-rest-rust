# InputStoryContentVideo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the content, must be *video* | [default to video]
**video** | **String** | The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass “attach://\\<file\\_attach\\_name\\>” if the video was uploaded using multipart/form-data under \\<file\\_attach\\_name\\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files) | 
**duration** | Option<**f64**> | *Optional*. Precise duration of the video in seconds; 0-60 | [optional]
**cover_frame_timestamp** | Option<**f64**> | *Optional*. Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0. | [optional]
**is_animation** | Option<**bool**> | *Optional*. Pass *True* if the video has no sound | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


