# InputSticker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sticker** | **String** | The added sticker. Pass a *file\\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass “attach://\\<file\\_attach\\_name\\>” to upload a new file using multipart/form-data under \\<file\\_attach\\_name\\> name. Animated and video stickers can't be uploaded via HTTP URL. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files) | 
**format** | **String** | Format of the added sticker, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, “video” for a **.WEBM** video | 
**emoji_list** | **Vec<String>** | List of 1-20 emoji associated with the sticker | 
**mask_position** | Option<[**models::MaskPosition**](MaskPosition.md)> |  | [optional]
**keywords** | Option<**Vec<String>**> | *Optional*. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom\\_emoji” stickers only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


