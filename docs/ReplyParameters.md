# ReplyParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_id** | **i32** | Identifier of the message that will be replied to in the current chat, or in the chat *chat\\_id* if it is specified | 
**chat_id** | Option<[**models::ReplyParametersChatId**](ReplyParameters_chat_id.md)> |  | [optional]
**allow_sending_without_reply** | Option<**bool**> | *Optional*. Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic. Always *True* for messages sent on behalf of a business account. | [optional]
**quote** | Option<**String**> | *Optional*. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\\_emoji* entities. The message will fail to send if the quote isn't found in the original message. | [optional]
**quote_parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**quote_entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote\\_parse\\_mode*. | [optional]
**quote_position** | Option<**i32**> | *Optional*. Position of the quote in the original message in UTF-16 code units | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


