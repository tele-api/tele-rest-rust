# InlineQueryResultContact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the result, must be *contact* | [default to contact]
**id** | **String** | Unique identifier for this result, 1-64 Bytes | 
**phone_number** | **String** | Contact's phone number | 
**first_name** | **String** | Contact's first name | 
**last_name** | Option<**String**> | *Optional*. Contact's last name | [optional]
**vcard** | Option<**String**> | *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes | [optional]
**reply_markup** | Option<[**models::InlineKeyboardMarkup**](InlineKeyboardMarkup.md)> |  | [optional]
**input_message_content** | Option<[**models::InputMessageContent**](InputMessageContent.md)> |  | [optional]
**thumbnail_url** | Option<**String**> | *Optional*. Url of the thumbnail for the result | [optional]
**thumbnail_width** | Option<**i32**> | *Optional*. Thumbnail width | [optional]
**thumbnail_height** | Option<**i32**> | *Optional*. Thumbnail height | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


