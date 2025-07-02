# CreateNewStickerSetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **i32** | User identifier of created sticker set owner | 
**name** | **String** | Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `\"_by_<bot_username>\"`. `<bot_username>` is case insensitive. 1-64 characters. | 
**title** | **String** | Sticker set title, 1-64 characters | 
**stickers** | [**Vec<models::InputSticker>**](InputSticker.md) | A JSON-serialized list of 1-50 initial stickers to be added to the sticker set | 
**sticker_type** | Option<**String**> | Type of stickers in the set, pass “regular”, “mask”, or “custom\\_emoji”. By default, a regular sticker set is created. | [optional]
**needs_repainting** | Option<**bool**> | Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


