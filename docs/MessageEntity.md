# MessageEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the entity. Currently, can be “mention” (`@username`), “hashtag” (`#hashtag` or `#hashtag@chatusername`), “cashtag” (`$USD` or `$USD@chatusername`), “bot\\_command” (`/start@jobs_bot`), “url” (`https://telegram.org`), “email” (`do-not-reply@telegram.org`), “phone\\_number” (`+1-212-555-0123`), “bold” (**bold text**), “italic” (*italic text*), “underline” (underlined text), “strikethrough” (strikethrough text), “spoiler” (spoiler message), “blockquote” (block quotation), “expandable\\_blockquote” (collapsed-by-default block quotation), “code” (monowidth string), “pre” (monowidth block), “text\\_link” (for clickable text URLs), “text\\_mention” (for users [without usernames](https://telegram.org/blog/edit#new-mentions)), “custom\\_emoji” (for inline custom emoji stickers) | 
**offset** | **i32** | Offset in [UTF-16 code units](https://core.telegram.org/api/entities#entity-length) to the start of the entity | 
**length** | **i32** | Length of the entity in [UTF-16 code units](https://core.telegram.org/api/entities#entity-length) | 
**url** | Option<**String**> | *Optional*. For “text\\_link” only, URL that will be opened after user taps on the text | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**language** | Option<**String**> | *Optional*. For “pre” only, the programming language of the entity text | [optional]
**custom_emoji_id** | Option<**String**> | *Optional*. For “custom\\_emoji” only, unique identifier of the custom emoji. Use [getCustomEmojiStickers](https://core.telegram.org/bots/api/#getcustomemojistickers) to get full information about the sticker | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


