# ExternalReplyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**origin** | [**models::MessageOrigin**](MessageOrigin.md) |  | 
**chat** | Option<[**models::Chat**](Chat.md)> |  | [optional]
**message_id** | Option<**i32**> | *Optional*. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel. | [optional]
**link_preview_options** | Option<[**models::LinkPreviewOptions**](LinkPreviewOptions.md)> |  | [optional]
**animation** | Option<[**models::Animation**](Animation.md)> |  | [optional]
**audio** | Option<[**models::Audio**](Audio.md)> |  | [optional]
**document** | Option<[**models::Document**](Document.md)> |  | [optional]
**paid_media** | Option<[**models::PaidMediaInfo**](PaidMediaInfo.md)> |  | [optional]
**photo** | Option<[**Vec<models::PhotoSize>**](PhotoSize.md)> | *Optional*. Message is a photo, available sizes of the photo | [optional]
**sticker** | Option<[**models::Sticker**](Sticker.md)> |  | [optional]
**story** | Option<[**models::Story**](Story.md)> |  | [optional]
**video** | Option<[**models::Video**](Video.md)> |  | [optional]
**video_note** | Option<[**models::VideoNote**](VideoNote.md)> |  | [optional]
**voice** | Option<[**models::Voice**](Voice.md)> |  | [optional]
**has_media_spoiler** | Option<**bool**> | *Optional*. *True*, if the message media is covered by a spoiler animation | [optional][default to true]
**contact** | Option<[**models::Contact**](Contact.md)> |  | [optional]
**dice** | Option<[**models::Dice**](Dice.md)> |  | [optional]
**game** | Option<[**models::Game**](Game.md)> |  | [optional]
**giveaway** | Option<[**models::Giveaway**](Giveaway.md)> |  | [optional]
**giveaway_winners** | Option<[**models::GiveawayWinners**](GiveawayWinners.md)> |  | [optional]
**invoice** | Option<[**models::Invoice**](Invoice.md)> |  | [optional]
**location** | Option<[**models::Location**](Location.md)> |  | [optional]
**poll** | Option<[**models::Poll**](Poll.md)> |  | [optional]
**venue** | Option<[**models::Venue**](Venue.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


