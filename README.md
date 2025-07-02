# Rust API client for tele_rest

The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram.
To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.

For more information, please visit [https://github.com/gitctrlx](https://github.com/gitctrlx)

## Overview

- API version: 9.0.0
- Package version: 9.0.0
- Build date: 2025-07-02T09:17:04.370667370Z[Etc/UTC]

## Installation

Put the package under your project folder in a directory named `tele_rest` and add the following to `Cargo.toml` under `[dependencies]`:

```
tele_rest = { path = "./tele_rest" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.telegram.org/bot123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**post_add_sticker_to_set**](docs/DefaultApi.md#post_add_sticker_to_set) | **POST** /addStickerToSet | addStickerToSet
*DefaultApi* | [**post_answer_callback_query**](docs/DefaultApi.md#post_answer_callback_query) | **POST** /answerCallbackQuery | answerCallbackQuery
*DefaultApi* | [**post_answer_inline_query**](docs/DefaultApi.md#post_answer_inline_query) | **POST** /answerInlineQuery | answerInlineQuery
*DefaultApi* | [**post_answer_pre_checkout_query**](docs/DefaultApi.md#post_answer_pre_checkout_query) | **POST** /answerPreCheckoutQuery | answerPreCheckoutQuery
*DefaultApi* | [**post_answer_shipping_query**](docs/DefaultApi.md#post_answer_shipping_query) | **POST** /answerShippingQuery | answerShippingQuery
*DefaultApi* | [**post_answer_web_app_query**](docs/DefaultApi.md#post_answer_web_app_query) | **POST** /answerWebAppQuery | answerWebAppQuery
*DefaultApi* | [**post_approve_chat_join_request**](docs/DefaultApi.md#post_approve_chat_join_request) | **POST** /approveChatJoinRequest | approveChatJoinRequest
*DefaultApi* | [**post_ban_chat_member**](docs/DefaultApi.md#post_ban_chat_member) | **POST** /banChatMember | banChatMember
*DefaultApi* | [**post_ban_chat_sender_chat**](docs/DefaultApi.md#post_ban_chat_sender_chat) | **POST** /banChatSenderChat | banChatSenderChat
*DefaultApi* | [**post_close**](docs/DefaultApi.md#post_close) | **POST** /close | close
*DefaultApi* | [**post_close_forum_topic**](docs/DefaultApi.md#post_close_forum_topic) | **POST** /closeForumTopic | closeForumTopic
*DefaultApi* | [**post_close_general_forum_topic**](docs/DefaultApi.md#post_close_general_forum_topic) | **POST** /closeGeneralForumTopic | closeGeneralForumTopic
*DefaultApi* | [**post_convert_gift_to_stars**](docs/DefaultApi.md#post_convert_gift_to_stars) | **POST** /convertGiftToStars | convertGiftToStars
*DefaultApi* | [**post_copy_message**](docs/DefaultApi.md#post_copy_message) | **POST** /copyMessage | copyMessage
*DefaultApi* | [**post_copy_messages**](docs/DefaultApi.md#post_copy_messages) | **POST** /copyMessages | copyMessages
*DefaultApi* | [**post_create_chat_invite_link**](docs/DefaultApi.md#post_create_chat_invite_link) | **POST** /createChatInviteLink | createChatInviteLink
*DefaultApi* | [**post_create_chat_subscription_invite_link**](docs/DefaultApi.md#post_create_chat_subscription_invite_link) | **POST** /createChatSubscriptionInviteLink | createChatSubscriptionInviteLink
*DefaultApi* | [**post_create_forum_topic**](docs/DefaultApi.md#post_create_forum_topic) | **POST** /createForumTopic | createForumTopic
*DefaultApi* | [**post_create_invoice_link**](docs/DefaultApi.md#post_create_invoice_link) | **POST** /createInvoiceLink | createInvoiceLink
*DefaultApi* | [**post_create_new_sticker_set**](docs/DefaultApi.md#post_create_new_sticker_set) | **POST** /createNewStickerSet | createNewStickerSet
*DefaultApi* | [**post_decline_chat_join_request**](docs/DefaultApi.md#post_decline_chat_join_request) | **POST** /declineChatJoinRequest | declineChatJoinRequest
*DefaultApi* | [**post_delete_business_messages**](docs/DefaultApi.md#post_delete_business_messages) | **POST** /deleteBusinessMessages | deleteBusinessMessages
*DefaultApi* | [**post_delete_chat_photo**](docs/DefaultApi.md#post_delete_chat_photo) | **POST** /deleteChatPhoto | deleteChatPhoto
*DefaultApi* | [**post_delete_chat_sticker_set**](docs/DefaultApi.md#post_delete_chat_sticker_set) | **POST** /deleteChatStickerSet | deleteChatStickerSet
*DefaultApi* | [**post_delete_forum_topic**](docs/DefaultApi.md#post_delete_forum_topic) | **POST** /deleteForumTopic | deleteForumTopic
*DefaultApi* | [**post_delete_message**](docs/DefaultApi.md#post_delete_message) | **POST** /deleteMessage | deleteMessage
*DefaultApi* | [**post_delete_messages**](docs/DefaultApi.md#post_delete_messages) | **POST** /deleteMessages | deleteMessages
*DefaultApi* | [**post_delete_my_commands**](docs/DefaultApi.md#post_delete_my_commands) | **POST** /deleteMyCommands | deleteMyCommands
*DefaultApi* | [**post_delete_sticker_from_set**](docs/DefaultApi.md#post_delete_sticker_from_set) | **POST** /deleteStickerFromSet | deleteStickerFromSet
*DefaultApi* | [**post_delete_sticker_set**](docs/DefaultApi.md#post_delete_sticker_set) | **POST** /deleteStickerSet | deleteStickerSet
*DefaultApi* | [**post_delete_story**](docs/DefaultApi.md#post_delete_story) | **POST** /deleteStory | deleteStory
*DefaultApi* | [**post_delete_webhook**](docs/DefaultApi.md#post_delete_webhook) | **POST** /deleteWebhook | deleteWebhook
*DefaultApi* | [**post_edit_chat_invite_link**](docs/DefaultApi.md#post_edit_chat_invite_link) | **POST** /editChatInviteLink | editChatInviteLink
*DefaultApi* | [**post_edit_chat_subscription_invite_link**](docs/DefaultApi.md#post_edit_chat_subscription_invite_link) | **POST** /editChatSubscriptionInviteLink | editChatSubscriptionInviteLink
*DefaultApi* | [**post_edit_forum_topic**](docs/DefaultApi.md#post_edit_forum_topic) | **POST** /editForumTopic | editForumTopic
*DefaultApi* | [**post_edit_general_forum_topic**](docs/DefaultApi.md#post_edit_general_forum_topic) | **POST** /editGeneralForumTopic | editGeneralForumTopic
*DefaultApi* | [**post_edit_message_caption**](docs/DefaultApi.md#post_edit_message_caption) | **POST** /editMessageCaption | editMessageCaption
*DefaultApi* | [**post_edit_message_live_location**](docs/DefaultApi.md#post_edit_message_live_location) | **POST** /editMessageLiveLocation | editMessageLiveLocation
*DefaultApi* | [**post_edit_message_media**](docs/DefaultApi.md#post_edit_message_media) | **POST** /editMessageMedia | editMessageMedia
*DefaultApi* | [**post_edit_message_reply_markup**](docs/DefaultApi.md#post_edit_message_reply_markup) | **POST** /editMessageReplyMarkup | editMessageReplyMarkup
*DefaultApi* | [**post_edit_message_text**](docs/DefaultApi.md#post_edit_message_text) | **POST** /editMessageText | editMessageText
*DefaultApi* | [**post_edit_story**](docs/DefaultApi.md#post_edit_story) | **POST** /editStory | editStory
*DefaultApi* | [**post_edit_user_star_subscription**](docs/DefaultApi.md#post_edit_user_star_subscription) | **POST** /editUserStarSubscription | editUserStarSubscription
*DefaultApi* | [**post_export_chat_invite_link**](docs/DefaultApi.md#post_export_chat_invite_link) | **POST** /exportChatInviteLink | exportChatInviteLink
*DefaultApi* | [**post_forward_message**](docs/DefaultApi.md#post_forward_message) | **POST** /forwardMessage | forwardMessage
*DefaultApi* | [**post_forward_messages**](docs/DefaultApi.md#post_forward_messages) | **POST** /forwardMessages | forwardMessages
*DefaultApi* | [**post_get_available_gifts**](docs/DefaultApi.md#post_get_available_gifts) | **POST** /getAvailableGifts | getAvailableGifts
*DefaultApi* | [**post_get_business_account_gifts**](docs/DefaultApi.md#post_get_business_account_gifts) | **POST** /getBusinessAccountGifts | getBusinessAccountGifts
*DefaultApi* | [**post_get_business_account_star_balance**](docs/DefaultApi.md#post_get_business_account_star_balance) | **POST** /getBusinessAccountStarBalance | getBusinessAccountStarBalance
*DefaultApi* | [**post_get_business_connection**](docs/DefaultApi.md#post_get_business_connection) | **POST** /getBusinessConnection | getBusinessConnection
*DefaultApi* | [**post_get_chat**](docs/DefaultApi.md#post_get_chat) | **POST** /getChat | getChat
*DefaultApi* | [**post_get_chat_administrators**](docs/DefaultApi.md#post_get_chat_administrators) | **POST** /getChatAdministrators | getChatAdministrators
*DefaultApi* | [**post_get_chat_member**](docs/DefaultApi.md#post_get_chat_member) | **POST** /getChatMember | getChatMember
*DefaultApi* | [**post_get_chat_member_count**](docs/DefaultApi.md#post_get_chat_member_count) | **POST** /getChatMemberCount | getChatMemberCount
*DefaultApi* | [**post_get_chat_menu_button**](docs/DefaultApi.md#post_get_chat_menu_button) | **POST** /getChatMenuButton | getChatMenuButton
*DefaultApi* | [**post_get_custom_emoji_stickers**](docs/DefaultApi.md#post_get_custom_emoji_stickers) | **POST** /getCustomEmojiStickers | getCustomEmojiStickers
*DefaultApi* | [**post_get_file**](docs/DefaultApi.md#post_get_file) | **POST** /getFile | getFile
*DefaultApi* | [**post_get_forum_topic_icon_stickers**](docs/DefaultApi.md#post_get_forum_topic_icon_stickers) | **POST** /getForumTopicIconStickers | getForumTopicIconStickers
*DefaultApi* | [**post_get_game_high_scores**](docs/DefaultApi.md#post_get_game_high_scores) | **POST** /getGameHighScores | getGameHighScores
*DefaultApi* | [**post_get_me**](docs/DefaultApi.md#post_get_me) | **POST** /getMe | getMe
*DefaultApi* | [**post_get_my_commands**](docs/DefaultApi.md#post_get_my_commands) | **POST** /getMyCommands | getMyCommands
*DefaultApi* | [**post_get_my_default_administrator_rights**](docs/DefaultApi.md#post_get_my_default_administrator_rights) | **POST** /getMyDefaultAdministratorRights | getMyDefaultAdministratorRights
*DefaultApi* | [**post_get_my_description**](docs/DefaultApi.md#post_get_my_description) | **POST** /getMyDescription | getMyDescription
*DefaultApi* | [**post_get_my_name**](docs/DefaultApi.md#post_get_my_name) | **POST** /getMyName | getMyName
*DefaultApi* | [**post_get_my_short_description**](docs/DefaultApi.md#post_get_my_short_description) | **POST** /getMyShortDescription | getMyShortDescription
*DefaultApi* | [**post_get_star_transactions**](docs/DefaultApi.md#post_get_star_transactions) | **POST** /getStarTransactions | getStarTransactions
*DefaultApi* | [**post_get_sticker_set**](docs/DefaultApi.md#post_get_sticker_set) | **POST** /getStickerSet | getStickerSet
*DefaultApi* | [**post_get_updates**](docs/DefaultApi.md#post_get_updates) | **POST** /getUpdates | getUpdates
*DefaultApi* | [**post_get_user_chat_boosts**](docs/DefaultApi.md#post_get_user_chat_boosts) | **POST** /getUserChatBoosts | getUserChatBoosts
*DefaultApi* | [**post_get_user_profile_photos**](docs/DefaultApi.md#post_get_user_profile_photos) | **POST** /getUserProfilePhotos | getUserProfilePhotos
*DefaultApi* | [**post_get_webhook_info**](docs/DefaultApi.md#post_get_webhook_info) | **POST** /getWebhookInfo | getWebhookInfo
*DefaultApi* | [**post_gift_premium_subscription**](docs/DefaultApi.md#post_gift_premium_subscription) | **POST** /giftPremiumSubscription | giftPremiumSubscription
*DefaultApi* | [**post_hide_general_forum_topic**](docs/DefaultApi.md#post_hide_general_forum_topic) | **POST** /hideGeneralForumTopic | hideGeneralForumTopic
*DefaultApi* | [**post_leave_chat**](docs/DefaultApi.md#post_leave_chat) | **POST** /leaveChat | leaveChat
*DefaultApi* | [**post_log_out**](docs/DefaultApi.md#post_log_out) | **POST** /logOut | logOut
*DefaultApi* | [**post_pin_chat_message**](docs/DefaultApi.md#post_pin_chat_message) | **POST** /pinChatMessage | pinChatMessage
*DefaultApi* | [**post_post_story**](docs/DefaultApi.md#post_post_story) | **POST** /postStory | postStory
*DefaultApi* | [**post_promote_chat_member**](docs/DefaultApi.md#post_promote_chat_member) | **POST** /promoteChatMember | promoteChatMember
*DefaultApi* | [**post_read_business_message**](docs/DefaultApi.md#post_read_business_message) | **POST** /readBusinessMessage | readBusinessMessage
*DefaultApi* | [**post_refund_star_payment**](docs/DefaultApi.md#post_refund_star_payment) | **POST** /refundStarPayment | refundStarPayment
*DefaultApi* | [**post_remove_business_account_profile_photo**](docs/DefaultApi.md#post_remove_business_account_profile_photo) | **POST** /removeBusinessAccountProfilePhoto | removeBusinessAccountProfilePhoto
*DefaultApi* | [**post_remove_chat_verification**](docs/DefaultApi.md#post_remove_chat_verification) | **POST** /removeChatVerification | removeChatVerification
*DefaultApi* | [**post_remove_user_verification**](docs/DefaultApi.md#post_remove_user_verification) | **POST** /removeUserVerification | removeUserVerification
*DefaultApi* | [**post_reopen_forum_topic**](docs/DefaultApi.md#post_reopen_forum_topic) | **POST** /reopenForumTopic | reopenForumTopic
*DefaultApi* | [**post_reopen_general_forum_topic**](docs/DefaultApi.md#post_reopen_general_forum_topic) | **POST** /reopenGeneralForumTopic | reopenGeneralForumTopic
*DefaultApi* | [**post_replace_sticker_in_set**](docs/DefaultApi.md#post_replace_sticker_in_set) | **POST** /replaceStickerInSet | replaceStickerInSet
*DefaultApi* | [**post_restrict_chat_member**](docs/DefaultApi.md#post_restrict_chat_member) | **POST** /restrictChatMember | restrictChatMember
*DefaultApi* | [**post_revoke_chat_invite_link**](docs/DefaultApi.md#post_revoke_chat_invite_link) | **POST** /revokeChatInviteLink | revokeChatInviteLink
*DefaultApi* | [**post_save_prepared_inline_message**](docs/DefaultApi.md#post_save_prepared_inline_message) | **POST** /savePreparedInlineMessage | savePreparedInlineMessage
*DefaultApi* | [**post_send_animation**](docs/DefaultApi.md#post_send_animation) | **POST** /sendAnimation | sendAnimation
*DefaultApi* | [**post_send_audio**](docs/DefaultApi.md#post_send_audio) | **POST** /sendAudio | sendAudio
*DefaultApi* | [**post_send_chat_action**](docs/DefaultApi.md#post_send_chat_action) | **POST** /sendChatAction | sendChatAction
*DefaultApi* | [**post_send_contact**](docs/DefaultApi.md#post_send_contact) | **POST** /sendContact | sendContact
*DefaultApi* | [**post_send_dice**](docs/DefaultApi.md#post_send_dice) | **POST** /sendDice | sendDice
*DefaultApi* | [**post_send_document**](docs/DefaultApi.md#post_send_document) | **POST** /sendDocument | sendDocument
*DefaultApi* | [**post_send_game**](docs/DefaultApi.md#post_send_game) | **POST** /sendGame | sendGame
*DefaultApi* | [**post_send_gift**](docs/DefaultApi.md#post_send_gift) | **POST** /sendGift | sendGift
*DefaultApi* | [**post_send_invoice**](docs/DefaultApi.md#post_send_invoice) | **POST** /sendInvoice | sendInvoice
*DefaultApi* | [**post_send_location**](docs/DefaultApi.md#post_send_location) | **POST** /sendLocation | sendLocation
*DefaultApi* | [**post_send_media_group**](docs/DefaultApi.md#post_send_media_group) | **POST** /sendMediaGroup | sendMediaGroup
*DefaultApi* | [**post_send_message**](docs/DefaultApi.md#post_send_message) | **POST** /sendMessage | sendMessage
*DefaultApi* | [**post_send_paid_media**](docs/DefaultApi.md#post_send_paid_media) | **POST** /sendPaidMedia | sendPaidMedia
*DefaultApi* | [**post_send_photo**](docs/DefaultApi.md#post_send_photo) | **POST** /sendPhoto | sendPhoto
*DefaultApi* | [**post_send_poll**](docs/DefaultApi.md#post_send_poll) | **POST** /sendPoll | sendPoll
*DefaultApi* | [**post_send_sticker**](docs/DefaultApi.md#post_send_sticker) | **POST** /sendSticker | sendSticker
*DefaultApi* | [**post_send_venue**](docs/DefaultApi.md#post_send_venue) | **POST** /sendVenue | sendVenue
*DefaultApi* | [**post_send_video**](docs/DefaultApi.md#post_send_video) | **POST** /sendVideo | sendVideo
*DefaultApi* | [**post_send_video_note**](docs/DefaultApi.md#post_send_video_note) | **POST** /sendVideoNote | sendVideoNote
*DefaultApi* | [**post_send_voice**](docs/DefaultApi.md#post_send_voice) | **POST** /sendVoice | sendVoice
*DefaultApi* | [**post_set_business_account_bio**](docs/DefaultApi.md#post_set_business_account_bio) | **POST** /setBusinessAccountBio | setBusinessAccountBio
*DefaultApi* | [**post_set_business_account_gift_settings**](docs/DefaultApi.md#post_set_business_account_gift_settings) | **POST** /setBusinessAccountGiftSettings | setBusinessAccountGiftSettings
*DefaultApi* | [**post_set_business_account_name**](docs/DefaultApi.md#post_set_business_account_name) | **POST** /setBusinessAccountName | setBusinessAccountName
*DefaultApi* | [**post_set_business_account_profile_photo**](docs/DefaultApi.md#post_set_business_account_profile_photo) | **POST** /setBusinessAccountProfilePhoto | setBusinessAccountProfilePhoto
*DefaultApi* | [**post_set_business_account_username**](docs/DefaultApi.md#post_set_business_account_username) | **POST** /setBusinessAccountUsername | setBusinessAccountUsername
*DefaultApi* | [**post_set_chat_administrator_custom_title**](docs/DefaultApi.md#post_set_chat_administrator_custom_title) | **POST** /setChatAdministratorCustomTitle | setChatAdministratorCustomTitle
*DefaultApi* | [**post_set_chat_description**](docs/DefaultApi.md#post_set_chat_description) | **POST** /setChatDescription | setChatDescription
*DefaultApi* | [**post_set_chat_menu_button**](docs/DefaultApi.md#post_set_chat_menu_button) | **POST** /setChatMenuButton | setChatMenuButton
*DefaultApi* | [**post_set_chat_permissions**](docs/DefaultApi.md#post_set_chat_permissions) | **POST** /setChatPermissions | setChatPermissions
*DefaultApi* | [**post_set_chat_photo**](docs/DefaultApi.md#post_set_chat_photo) | **POST** /setChatPhoto | setChatPhoto
*DefaultApi* | [**post_set_chat_sticker_set**](docs/DefaultApi.md#post_set_chat_sticker_set) | **POST** /setChatStickerSet | setChatStickerSet
*DefaultApi* | [**post_set_chat_title**](docs/DefaultApi.md#post_set_chat_title) | **POST** /setChatTitle | setChatTitle
*DefaultApi* | [**post_set_custom_emoji_sticker_set_thumbnail**](docs/DefaultApi.md#post_set_custom_emoji_sticker_set_thumbnail) | **POST** /setCustomEmojiStickerSetThumbnail | setCustomEmojiStickerSetThumbnail
*DefaultApi* | [**post_set_game_score**](docs/DefaultApi.md#post_set_game_score) | **POST** /setGameScore | setGameScore
*DefaultApi* | [**post_set_message_reaction**](docs/DefaultApi.md#post_set_message_reaction) | **POST** /setMessageReaction | setMessageReaction
*DefaultApi* | [**post_set_my_commands**](docs/DefaultApi.md#post_set_my_commands) | **POST** /setMyCommands | setMyCommands
*DefaultApi* | [**post_set_my_default_administrator_rights**](docs/DefaultApi.md#post_set_my_default_administrator_rights) | **POST** /setMyDefaultAdministratorRights | setMyDefaultAdministratorRights
*DefaultApi* | [**post_set_my_description**](docs/DefaultApi.md#post_set_my_description) | **POST** /setMyDescription | setMyDescription
*DefaultApi* | [**post_set_my_name**](docs/DefaultApi.md#post_set_my_name) | **POST** /setMyName | setMyName
*DefaultApi* | [**post_set_my_short_description**](docs/DefaultApi.md#post_set_my_short_description) | **POST** /setMyShortDescription | setMyShortDescription
*DefaultApi* | [**post_set_passport_data_errors**](docs/DefaultApi.md#post_set_passport_data_errors) | **POST** /setPassportDataErrors | setPassportDataErrors
*DefaultApi* | [**post_set_sticker_emoji_list**](docs/DefaultApi.md#post_set_sticker_emoji_list) | **POST** /setStickerEmojiList | setStickerEmojiList
*DefaultApi* | [**post_set_sticker_keywords**](docs/DefaultApi.md#post_set_sticker_keywords) | **POST** /setStickerKeywords | setStickerKeywords
*DefaultApi* | [**post_set_sticker_mask_position**](docs/DefaultApi.md#post_set_sticker_mask_position) | **POST** /setStickerMaskPosition | setStickerMaskPosition
*DefaultApi* | [**post_set_sticker_position_in_set**](docs/DefaultApi.md#post_set_sticker_position_in_set) | **POST** /setStickerPositionInSet | setStickerPositionInSet
*DefaultApi* | [**post_set_sticker_set_thumbnail**](docs/DefaultApi.md#post_set_sticker_set_thumbnail) | **POST** /setStickerSetThumbnail | setStickerSetThumbnail
*DefaultApi* | [**post_set_sticker_set_title**](docs/DefaultApi.md#post_set_sticker_set_title) | **POST** /setStickerSetTitle | setStickerSetTitle
*DefaultApi* | [**post_set_user_emoji_status**](docs/DefaultApi.md#post_set_user_emoji_status) | **POST** /setUserEmojiStatus | setUserEmojiStatus
*DefaultApi* | [**post_set_webhook**](docs/DefaultApi.md#post_set_webhook) | **POST** /setWebhook | setWebhook
*DefaultApi* | [**post_stop_message_live_location**](docs/DefaultApi.md#post_stop_message_live_location) | **POST** /stopMessageLiveLocation | stopMessageLiveLocation
*DefaultApi* | [**post_stop_poll**](docs/DefaultApi.md#post_stop_poll) | **POST** /stopPoll | stopPoll
*DefaultApi* | [**post_transfer_business_account_stars**](docs/DefaultApi.md#post_transfer_business_account_stars) | **POST** /transferBusinessAccountStars | transferBusinessAccountStars
*DefaultApi* | [**post_transfer_gift**](docs/DefaultApi.md#post_transfer_gift) | **POST** /transferGift | transferGift
*DefaultApi* | [**post_unban_chat_member**](docs/DefaultApi.md#post_unban_chat_member) | **POST** /unbanChatMember | unbanChatMember
*DefaultApi* | [**post_unban_chat_sender_chat**](docs/DefaultApi.md#post_unban_chat_sender_chat) | **POST** /unbanChatSenderChat | unbanChatSenderChat
*DefaultApi* | [**post_unhide_general_forum_topic**](docs/DefaultApi.md#post_unhide_general_forum_topic) | **POST** /unhideGeneralForumTopic | unhideGeneralForumTopic
*DefaultApi* | [**post_unpin_all_chat_messages**](docs/DefaultApi.md#post_unpin_all_chat_messages) | **POST** /unpinAllChatMessages | unpinAllChatMessages
*DefaultApi* | [**post_unpin_all_forum_topic_messages**](docs/DefaultApi.md#post_unpin_all_forum_topic_messages) | **POST** /unpinAllForumTopicMessages | unpinAllForumTopicMessages
*DefaultApi* | [**post_unpin_all_general_forum_topic_messages**](docs/DefaultApi.md#post_unpin_all_general_forum_topic_messages) | **POST** /unpinAllGeneralForumTopicMessages | unpinAllGeneralForumTopicMessages
*DefaultApi* | [**post_unpin_chat_message**](docs/DefaultApi.md#post_unpin_chat_message) | **POST** /unpinChatMessage | unpinChatMessage
*DefaultApi* | [**post_upgrade_gift**](docs/DefaultApi.md#post_upgrade_gift) | **POST** /upgradeGift | upgradeGift
*DefaultApi* | [**post_upload_sticker_file**](docs/DefaultApi.md#post_upload_sticker_file) | **POST** /uploadStickerFile | uploadStickerFile
*DefaultApi* | [**post_verify_chat**](docs/DefaultApi.md#post_verify_chat) | **POST** /verifyChat | verifyChat
*DefaultApi* | [**post_verify_user**](docs/DefaultApi.md#post_verify_user) | **POST** /verifyUser | verifyUser


## Documentation For Models

 - [AcceptedGiftTypes](docs/AcceptedGiftTypes.md)
 - [AddStickerToSetRequest](docs/AddStickerToSetRequest.md)
 - [AddStickerToSetResponse](docs/AddStickerToSetResponse.md)
 - [AffiliateInfo](docs/AffiliateInfo.md)
 - [Animation](docs/Animation.md)
 - [AnswerCallbackQueryRequest](docs/AnswerCallbackQueryRequest.md)
 - [AnswerCallbackQueryResponse](docs/AnswerCallbackQueryResponse.md)
 - [AnswerInlineQueryRequest](docs/AnswerInlineQueryRequest.md)
 - [AnswerInlineQueryResponse](docs/AnswerInlineQueryResponse.md)
 - [AnswerPreCheckoutQueryRequest](docs/AnswerPreCheckoutQueryRequest.md)
 - [AnswerPreCheckoutQueryResponse](docs/AnswerPreCheckoutQueryResponse.md)
 - [AnswerShippingQueryRequest](docs/AnswerShippingQueryRequest.md)
 - [AnswerShippingQueryResponse](docs/AnswerShippingQueryResponse.md)
 - [AnswerWebAppQueryRequest](docs/AnswerWebAppQueryRequest.md)
 - [AnswerWebAppQueryResponse](docs/AnswerWebAppQueryResponse.md)
 - [ApproveChatJoinRequestRequest](docs/ApproveChatJoinRequestRequest.md)
 - [ApproveChatJoinRequestResponse](docs/ApproveChatJoinRequestResponse.md)
 - [Audio](docs/Audio.md)
 - [BackgroundFill](docs/BackgroundFill.md)
 - [BackgroundFillFreeformGradient](docs/BackgroundFillFreeformGradient.md)
 - [BackgroundFillGradient](docs/BackgroundFillGradient.md)
 - [BackgroundFillSolid](docs/BackgroundFillSolid.md)
 - [BackgroundType](docs/BackgroundType.md)
 - [BackgroundTypeChatTheme](docs/BackgroundTypeChatTheme.md)
 - [BackgroundTypeFill](docs/BackgroundTypeFill.md)
 - [BackgroundTypePattern](docs/BackgroundTypePattern.md)
 - [BackgroundTypeWallpaper](docs/BackgroundTypeWallpaper.md)
 - [BanChatMemberRequest](docs/BanChatMemberRequest.md)
 - [BanChatMemberRequestChatId](docs/BanChatMemberRequestChatId.md)
 - [BanChatMemberResponse](docs/BanChatMemberResponse.md)
 - [BanChatSenderChatRequest](docs/BanChatSenderChatRequest.md)
 - [BanChatSenderChatResponse](docs/BanChatSenderChatResponse.md)
 - [Birthdate](docs/Birthdate.md)
 - [BotCommand](docs/BotCommand.md)
 - [BotCommandScope](docs/BotCommandScope.md)
 - [BotCommandScopeAllChatAdministrators](docs/BotCommandScopeAllChatAdministrators.md)
 - [BotCommandScopeAllGroupChats](docs/BotCommandScopeAllGroupChats.md)
 - [BotCommandScopeAllPrivateChats](docs/BotCommandScopeAllPrivateChats.md)
 - [BotCommandScopeChat](docs/BotCommandScopeChat.md)
 - [BotCommandScopeChatAdministrators](docs/BotCommandScopeChatAdministrators.md)
 - [BotCommandScopeChatChatId](docs/BotCommandScopeChatChatId.md)
 - [BotCommandScopeChatMember](docs/BotCommandScopeChatMember.md)
 - [BotCommandScopeDefault](docs/BotCommandScopeDefault.md)
 - [BotDescription](docs/BotDescription.md)
 - [BotName](docs/BotName.md)
 - [BotShortDescription](docs/BotShortDescription.md)
 - [BusinessBotRights](docs/BusinessBotRights.md)
 - [BusinessConnection](docs/BusinessConnection.md)
 - [BusinessIntro](docs/BusinessIntro.md)
 - [BusinessLocation](docs/BusinessLocation.md)
 - [BusinessMessagesDeleted](docs/BusinessMessagesDeleted.md)
 - [BusinessOpeningHours](docs/BusinessOpeningHours.md)
 - [BusinessOpeningHoursInterval](docs/BusinessOpeningHoursInterval.md)
 - [CallbackQuery](docs/CallbackQuery.md)
 - [Chat](docs/Chat.md)
 - [ChatAdministratorRights](docs/ChatAdministratorRights.md)
 - [ChatBackground](docs/ChatBackground.md)
 - [ChatBoost](docs/ChatBoost.md)
 - [ChatBoostAdded](docs/ChatBoostAdded.md)
 - [ChatBoostRemoved](docs/ChatBoostRemoved.md)
 - [ChatBoostSource](docs/ChatBoostSource.md)
 - [ChatBoostSourceGiftCode](docs/ChatBoostSourceGiftCode.md)
 - [ChatBoostSourceGiveaway](docs/ChatBoostSourceGiveaway.md)
 - [ChatBoostSourcePremium](docs/ChatBoostSourcePremium.md)
 - [ChatBoostUpdated](docs/ChatBoostUpdated.md)
 - [ChatFullInfo](docs/ChatFullInfo.md)
 - [ChatInviteLink](docs/ChatInviteLink.md)
 - [ChatJoinRequest](docs/ChatJoinRequest.md)
 - [ChatLocation](docs/ChatLocation.md)
 - [ChatMember](docs/ChatMember.md)
 - [ChatMemberAdministrator](docs/ChatMemberAdministrator.md)
 - [ChatMemberBanned](docs/ChatMemberBanned.md)
 - [ChatMemberLeft](docs/ChatMemberLeft.md)
 - [ChatMemberMember](docs/ChatMemberMember.md)
 - [ChatMemberOwner](docs/ChatMemberOwner.md)
 - [ChatMemberRestricted](docs/ChatMemberRestricted.md)
 - [ChatMemberUpdated](docs/ChatMemberUpdated.md)
 - [ChatPermissions](docs/ChatPermissions.md)
 - [ChatPhoto](docs/ChatPhoto.md)
 - [ChatShared](docs/ChatShared.md)
 - [ChosenInlineResult](docs/ChosenInlineResult.md)
 - [CloseForumTopicRequest](docs/CloseForumTopicRequest.md)
 - [CloseForumTopicResponse](docs/CloseForumTopicResponse.md)
 - [CloseGeneralForumTopicRequest](docs/CloseGeneralForumTopicRequest.md)
 - [CloseGeneralForumTopicResponse](docs/CloseGeneralForumTopicResponse.md)
 - [CloseResponse](docs/CloseResponse.md)
 - [Contact](docs/Contact.md)
 - [ConvertGiftToStarsRequest](docs/ConvertGiftToStarsRequest.md)
 - [ConvertGiftToStarsResponse](docs/ConvertGiftToStarsResponse.md)
 - [CopyMessageRequest](docs/CopyMessageRequest.md)
 - [CopyMessageResponse](docs/CopyMessageResponse.md)
 - [CopyMessagesRequest](docs/CopyMessagesRequest.md)
 - [CopyMessagesResponse](docs/CopyMessagesResponse.md)
 - [CopyTextButton](docs/CopyTextButton.md)
 - [CreateChatInviteLinkRequest](docs/CreateChatInviteLinkRequest.md)
 - [CreateChatInviteLinkResponse](docs/CreateChatInviteLinkResponse.md)
 - [CreateChatSubscriptionInviteLinkRequest](docs/CreateChatSubscriptionInviteLinkRequest.md)
 - [CreateChatSubscriptionInviteLinkRequestChatId](docs/CreateChatSubscriptionInviteLinkRequestChatId.md)
 - [CreateChatSubscriptionInviteLinkResponse](docs/CreateChatSubscriptionInviteLinkResponse.md)
 - [CreateForumTopicRequest](docs/CreateForumTopicRequest.md)
 - [CreateForumTopicResponse](docs/CreateForumTopicResponse.md)
 - [CreateInvoiceLinkRequest](docs/CreateInvoiceLinkRequest.md)
 - [CreateInvoiceLinkResponse](docs/CreateInvoiceLinkResponse.md)
 - [CreateNewStickerSetRequest](docs/CreateNewStickerSetRequest.md)
 - [CreateNewStickerSetResponse](docs/CreateNewStickerSetResponse.md)
 - [DeclineChatJoinRequestRequest](docs/DeclineChatJoinRequestRequest.md)
 - [DeclineChatJoinRequestResponse](docs/DeclineChatJoinRequestResponse.md)
 - [DeleteBusinessMessagesRequest](docs/DeleteBusinessMessagesRequest.md)
 - [DeleteBusinessMessagesResponse](docs/DeleteBusinessMessagesResponse.md)
 - [DeleteChatPhotoRequest](docs/DeleteChatPhotoRequest.md)
 - [DeleteChatPhotoResponse](docs/DeleteChatPhotoResponse.md)
 - [DeleteChatStickerSetRequest](docs/DeleteChatStickerSetRequest.md)
 - [DeleteChatStickerSetResponse](docs/DeleteChatStickerSetResponse.md)
 - [DeleteForumTopicRequest](docs/DeleteForumTopicRequest.md)
 - [DeleteForumTopicResponse](docs/DeleteForumTopicResponse.md)
 - [DeleteMessageRequest](docs/DeleteMessageRequest.md)
 - [DeleteMessageResponse](docs/DeleteMessageResponse.md)
 - [DeleteMessagesRequest](docs/DeleteMessagesRequest.md)
 - [DeleteMessagesResponse](docs/DeleteMessagesResponse.md)
 - [DeleteMyCommandsRequest](docs/DeleteMyCommandsRequest.md)
 - [DeleteMyCommandsResponse](docs/DeleteMyCommandsResponse.md)
 - [DeleteStickerFromSetRequest](docs/DeleteStickerFromSetRequest.md)
 - [DeleteStickerFromSetResponse](docs/DeleteStickerFromSetResponse.md)
 - [DeleteStickerSetRequest](docs/DeleteStickerSetRequest.md)
 - [DeleteStickerSetResponse](docs/DeleteStickerSetResponse.md)
 - [DeleteStoryRequest](docs/DeleteStoryRequest.md)
 - [DeleteStoryResponse](docs/DeleteStoryResponse.md)
 - [DeleteWebhookRequest](docs/DeleteWebhookRequest.md)
 - [DeleteWebhookResponse](docs/DeleteWebhookResponse.md)
 - [Dice](docs/Dice.md)
 - [Document](docs/Document.md)
 - [EditChatInviteLinkRequest](docs/EditChatInviteLinkRequest.md)
 - [EditChatInviteLinkResponse](docs/EditChatInviteLinkResponse.md)
 - [EditChatSubscriptionInviteLinkRequest](docs/EditChatSubscriptionInviteLinkRequest.md)
 - [EditChatSubscriptionInviteLinkResponse](docs/EditChatSubscriptionInviteLinkResponse.md)
 - [EditForumTopicRequest](docs/EditForumTopicRequest.md)
 - [EditForumTopicResponse](docs/EditForumTopicResponse.md)
 - [EditGeneralForumTopicRequest](docs/EditGeneralForumTopicRequest.md)
 - [EditGeneralForumTopicResponse](docs/EditGeneralForumTopicResponse.md)
 - [EditMessageCaptionRequest](docs/EditMessageCaptionRequest.md)
 - [EditMessageCaptionResponse](docs/EditMessageCaptionResponse.md)
 - [EditMessageLiveLocationRequest](docs/EditMessageLiveLocationRequest.md)
 - [EditMessageLiveLocationResponse](docs/EditMessageLiveLocationResponse.md)
 - [EditMessageMediaRequest](docs/EditMessageMediaRequest.md)
 - [EditMessageMediaResponse](docs/EditMessageMediaResponse.md)
 - [EditMessageReplyMarkupRequest](docs/EditMessageReplyMarkupRequest.md)
 - [EditMessageReplyMarkupResponse](docs/EditMessageReplyMarkupResponse.md)
 - [EditMessageTextRequest](docs/EditMessageTextRequest.md)
 - [EditMessageTextRequestChatId](docs/EditMessageTextRequestChatId.md)
 - [EditMessageTextResponse](docs/EditMessageTextResponse.md)
 - [EditMessageTextResponseResult](docs/EditMessageTextResponseResult.md)
 - [EditStoryRequest](docs/EditStoryRequest.md)
 - [EditStoryResponse](docs/EditStoryResponse.md)
 - [EditUserStarSubscriptionRequest](docs/EditUserStarSubscriptionRequest.md)
 - [EditUserStarSubscriptionResponse](docs/EditUserStarSubscriptionResponse.md)
 - [EncryptedCredentials](docs/EncryptedCredentials.md)
 - [EncryptedPassportElement](docs/EncryptedPassportElement.md)
 - [Error](docs/Error.md)
 - [ExportChatInviteLinkRequest](docs/ExportChatInviteLinkRequest.md)
 - [ExportChatInviteLinkResponse](docs/ExportChatInviteLinkResponse.md)
 - [ExternalReplyInfo](docs/ExternalReplyInfo.md)
 - [File](docs/File.md)
 - [ForceReply](docs/ForceReply.md)
 - [ForumTopic](docs/ForumTopic.md)
 - [ForumTopicCreated](docs/ForumTopicCreated.md)
 - [ForumTopicEdited](docs/ForumTopicEdited.md)
 - [ForwardMessageRequest](docs/ForwardMessageRequest.md)
 - [ForwardMessageRequestFromChatId](docs/ForwardMessageRequestFromChatId.md)
 - [ForwardMessageResponse](docs/ForwardMessageResponse.md)
 - [ForwardMessagesRequest](docs/ForwardMessagesRequest.md)
 - [ForwardMessagesRequestFromChatId](docs/ForwardMessagesRequestFromChatId.md)
 - [ForwardMessagesResponse](docs/ForwardMessagesResponse.md)
 - [Game](docs/Game.md)
 - [GameHighScore](docs/GameHighScore.md)
 - [GetAvailableGiftsResponse](docs/GetAvailableGiftsResponse.md)
 - [GetBusinessAccountGiftsRequest](docs/GetBusinessAccountGiftsRequest.md)
 - [GetBusinessAccountGiftsResponse](docs/GetBusinessAccountGiftsResponse.md)
 - [GetBusinessAccountStarBalanceRequest](docs/GetBusinessAccountStarBalanceRequest.md)
 - [GetBusinessAccountStarBalanceResponse](docs/GetBusinessAccountStarBalanceResponse.md)
 - [GetBusinessConnectionRequest](docs/GetBusinessConnectionRequest.md)
 - [GetBusinessConnectionResponse](docs/GetBusinessConnectionResponse.md)
 - [GetChatAdministratorsRequest](docs/GetChatAdministratorsRequest.md)
 - [GetChatAdministratorsResponse](docs/GetChatAdministratorsResponse.md)
 - [GetChatMemberCountRequest](docs/GetChatMemberCountRequest.md)
 - [GetChatMemberCountResponse](docs/GetChatMemberCountResponse.md)
 - [GetChatMemberRequest](docs/GetChatMemberRequest.md)
 - [GetChatMemberResponse](docs/GetChatMemberResponse.md)
 - [GetChatMenuButtonRequest](docs/GetChatMenuButtonRequest.md)
 - [GetChatMenuButtonResponse](docs/GetChatMenuButtonResponse.md)
 - [GetChatRequest](docs/GetChatRequest.md)
 - [GetChatResponse](docs/GetChatResponse.md)
 - [GetCustomEmojiStickersRequest](docs/GetCustomEmojiStickersRequest.md)
 - [GetCustomEmojiStickersResponse](docs/GetCustomEmojiStickersResponse.md)
 - [GetFileRequest](docs/GetFileRequest.md)
 - [GetFileResponse](docs/GetFileResponse.md)
 - [GetForumTopicIconStickersResponse](docs/GetForumTopicIconStickersResponse.md)
 - [GetGameHighScoresRequest](docs/GetGameHighScoresRequest.md)
 - [GetGameHighScoresResponse](docs/GetGameHighScoresResponse.md)
 - [GetMeResponse](docs/GetMeResponse.md)
 - [GetMyCommandsRequest](docs/GetMyCommandsRequest.md)
 - [GetMyCommandsResponse](docs/GetMyCommandsResponse.md)
 - [GetMyDefaultAdministratorRightsRequest](docs/GetMyDefaultAdministratorRightsRequest.md)
 - [GetMyDefaultAdministratorRightsResponse](docs/GetMyDefaultAdministratorRightsResponse.md)
 - [GetMyDescriptionRequest](docs/GetMyDescriptionRequest.md)
 - [GetMyDescriptionResponse](docs/GetMyDescriptionResponse.md)
 - [GetMyNameRequest](docs/GetMyNameRequest.md)
 - [GetMyNameResponse](docs/GetMyNameResponse.md)
 - [GetMyShortDescriptionRequest](docs/GetMyShortDescriptionRequest.md)
 - [GetMyShortDescriptionResponse](docs/GetMyShortDescriptionResponse.md)
 - [GetStarTransactionsRequest](docs/GetStarTransactionsRequest.md)
 - [GetStarTransactionsResponse](docs/GetStarTransactionsResponse.md)
 - [GetStickerSetRequest](docs/GetStickerSetRequest.md)
 - [GetStickerSetResponse](docs/GetStickerSetResponse.md)
 - [GetUpdatesRequest](docs/GetUpdatesRequest.md)
 - [GetUpdatesResponse](docs/GetUpdatesResponse.md)
 - [GetUserChatBoostsRequest](docs/GetUserChatBoostsRequest.md)
 - [GetUserChatBoostsRequestChatId](docs/GetUserChatBoostsRequestChatId.md)
 - [GetUserChatBoostsResponse](docs/GetUserChatBoostsResponse.md)
 - [GetUserProfilePhotosRequest](docs/GetUserProfilePhotosRequest.md)
 - [GetUserProfilePhotosResponse](docs/GetUserProfilePhotosResponse.md)
 - [GetWebhookInfoResponse](docs/GetWebhookInfoResponse.md)
 - [Gift](docs/Gift.md)
 - [GiftInfo](docs/GiftInfo.md)
 - [GiftPremiumSubscriptionRequest](docs/GiftPremiumSubscriptionRequest.md)
 - [GiftPremiumSubscriptionResponse](docs/GiftPremiumSubscriptionResponse.md)
 - [Gifts](docs/Gifts.md)
 - [Giveaway](docs/Giveaway.md)
 - [GiveawayCompleted](docs/GiveawayCompleted.md)
 - [GiveawayCreated](docs/GiveawayCreated.md)
 - [GiveawayWinners](docs/GiveawayWinners.md)
 - [HideGeneralForumTopicRequest](docs/HideGeneralForumTopicRequest.md)
 - [HideGeneralForumTopicResponse](docs/HideGeneralForumTopicResponse.md)
 - [InaccessibleMessage](docs/InaccessibleMessage.md)
 - [InlineKeyboardButton](docs/InlineKeyboardButton.md)
 - [InlineKeyboardMarkup](docs/InlineKeyboardMarkup.md)
 - [InlineQuery](docs/InlineQuery.md)
 - [InlineQueryResult](docs/InlineQueryResult.md)
 - [InlineQueryResultArticle](docs/InlineQueryResultArticle.md)
 - [InlineQueryResultAudio](docs/InlineQueryResultAudio.md)
 - [InlineQueryResultCachedAudio](docs/InlineQueryResultCachedAudio.md)
 - [InlineQueryResultCachedDocument](docs/InlineQueryResultCachedDocument.md)
 - [InlineQueryResultCachedGif](docs/InlineQueryResultCachedGif.md)
 - [InlineQueryResultCachedMpeg4Gif](docs/InlineQueryResultCachedMpeg4Gif.md)
 - [InlineQueryResultCachedPhoto](docs/InlineQueryResultCachedPhoto.md)
 - [InlineQueryResultCachedSticker](docs/InlineQueryResultCachedSticker.md)
 - [InlineQueryResultCachedVideo](docs/InlineQueryResultCachedVideo.md)
 - [InlineQueryResultCachedVoice](docs/InlineQueryResultCachedVoice.md)
 - [InlineQueryResultContact](docs/InlineQueryResultContact.md)
 - [InlineQueryResultDocument](docs/InlineQueryResultDocument.md)
 - [InlineQueryResultGame](docs/InlineQueryResultGame.md)
 - [InlineQueryResultGif](docs/InlineQueryResultGif.md)
 - [InlineQueryResultLocation](docs/InlineQueryResultLocation.md)
 - [InlineQueryResultMpeg4Gif](docs/InlineQueryResultMpeg4Gif.md)
 - [InlineQueryResultPhoto](docs/InlineQueryResultPhoto.md)
 - [InlineQueryResultVenue](docs/InlineQueryResultVenue.md)
 - [InlineQueryResultVideo](docs/InlineQueryResultVideo.md)
 - [InlineQueryResultVoice](docs/InlineQueryResultVoice.md)
 - [InlineQueryResultsButton](docs/InlineQueryResultsButton.md)
 - [InputContactMessageContent](docs/InputContactMessageContent.md)
 - [InputInvoiceMessageContent](docs/InputInvoiceMessageContent.md)
 - [InputLocationMessageContent](docs/InputLocationMessageContent.md)
 - [InputMedia](docs/InputMedia.md)
 - [InputMediaAnimation](docs/InputMediaAnimation.md)
 - [InputMediaAudio](docs/InputMediaAudio.md)
 - [InputMediaDocument](docs/InputMediaDocument.md)
 - [InputMediaPhoto](docs/InputMediaPhoto.md)
 - [InputMediaVideo](docs/InputMediaVideo.md)
 - [InputMessageContent](docs/InputMessageContent.md)
 - [InputPaidMedia](docs/InputPaidMedia.md)
 - [InputPaidMediaPhoto](docs/InputPaidMediaPhoto.md)
 - [InputPaidMediaVideo](docs/InputPaidMediaVideo.md)
 - [InputPollOption](docs/InputPollOption.md)
 - [InputProfilePhoto](docs/InputProfilePhoto.md)
 - [InputProfilePhotoAnimated](docs/InputProfilePhotoAnimated.md)
 - [InputProfilePhotoStatic](docs/InputProfilePhotoStatic.md)
 - [InputSticker](docs/InputSticker.md)
 - [InputStoryContent](docs/InputStoryContent.md)
 - [InputStoryContentPhoto](docs/InputStoryContentPhoto.md)
 - [InputStoryContentVideo](docs/InputStoryContentVideo.md)
 - [InputTextMessageContent](docs/InputTextMessageContent.md)
 - [InputVenueMessageContent](docs/InputVenueMessageContent.md)
 - [Invoice](docs/Invoice.md)
 - [KeyboardButton](docs/KeyboardButton.md)
 - [KeyboardButtonPollType](docs/KeyboardButtonPollType.md)
 - [KeyboardButtonRequestChat](docs/KeyboardButtonRequestChat.md)
 - [KeyboardButtonRequestUsers](docs/KeyboardButtonRequestUsers.md)
 - [LabeledPrice](docs/LabeledPrice.md)
 - [LeaveChatRequest](docs/LeaveChatRequest.md)
 - [LeaveChatRequestChatId](docs/LeaveChatRequestChatId.md)
 - [LeaveChatResponse](docs/LeaveChatResponse.md)
 - [LinkPreviewOptions](docs/LinkPreviewOptions.md)
 - [Location](docs/Location.md)
 - [LocationAddress](docs/LocationAddress.md)
 - [LogOutResponse](docs/LogOutResponse.md)
 - [LoginUrl](docs/LoginUrl.md)
 - [MaskPosition](docs/MaskPosition.md)
 - [MaybeInaccessibleMessage](docs/MaybeInaccessibleMessage.md)
 - [MenuButton](docs/MenuButton.md)
 - [MenuButtonCommands](docs/MenuButtonCommands.md)
 - [MenuButtonDefault](docs/MenuButtonDefault.md)
 - [MenuButtonWebApp](docs/MenuButtonWebApp.md)
 - [Message](docs/Message.md)
 - [MessageAutoDeleteTimerChanged](docs/MessageAutoDeleteTimerChanged.md)
 - [MessageEntity](docs/MessageEntity.md)
 - [MessageId](docs/MessageId.md)
 - [MessageOrigin](docs/MessageOrigin.md)
 - [MessageOriginChannel](docs/MessageOriginChannel.md)
 - [MessageOriginChat](docs/MessageOriginChat.md)
 - [MessageOriginHiddenUser](docs/MessageOriginHiddenUser.md)
 - [MessageOriginUser](docs/MessageOriginUser.md)
 - [MessageReactionCountUpdated](docs/MessageReactionCountUpdated.md)
 - [MessageReactionUpdated](docs/MessageReactionUpdated.md)
 - [OrderInfo](docs/OrderInfo.md)
 - [OwnedGift](docs/OwnedGift.md)
 - [OwnedGiftRegular](docs/OwnedGiftRegular.md)
 - [OwnedGiftUnique](docs/OwnedGiftUnique.md)
 - [OwnedGifts](docs/OwnedGifts.md)
 - [PaidMedia](docs/PaidMedia.md)
 - [PaidMediaInfo](docs/PaidMediaInfo.md)
 - [PaidMediaPhoto](docs/PaidMediaPhoto.md)
 - [PaidMediaPreview](docs/PaidMediaPreview.md)
 - [PaidMediaPurchased](docs/PaidMediaPurchased.md)
 - [PaidMediaVideo](docs/PaidMediaVideo.md)
 - [PaidMessagePriceChanged](docs/PaidMessagePriceChanged.md)
 - [PassportData](docs/PassportData.md)
 - [PassportElementError](docs/PassportElementError.md)
 - [PassportElementErrorDataField](docs/PassportElementErrorDataField.md)
 - [PassportElementErrorFile](docs/PassportElementErrorFile.md)
 - [PassportElementErrorFiles](docs/PassportElementErrorFiles.md)
 - [PassportElementErrorFrontSide](docs/PassportElementErrorFrontSide.md)
 - [PassportElementErrorReverseSide](docs/PassportElementErrorReverseSide.md)
 - [PassportElementErrorSelfie](docs/PassportElementErrorSelfie.md)
 - [PassportElementErrorTranslationFile](docs/PassportElementErrorTranslationFile.md)
 - [PassportElementErrorTranslationFiles](docs/PassportElementErrorTranslationFiles.md)
 - [PassportElementErrorUnspecified](docs/PassportElementErrorUnspecified.md)
 - [PassportFile](docs/PassportFile.md)
 - [PhotoSize](docs/PhotoSize.md)
 - [PinChatMessageRequest](docs/PinChatMessageRequest.md)
 - [PinChatMessageResponse](docs/PinChatMessageResponse.md)
 - [Poll](docs/Poll.md)
 - [PollAnswer](docs/PollAnswer.md)
 - [PollOption](docs/PollOption.md)
 - [PostStoryRequest](docs/PostStoryRequest.md)
 - [PostStoryResponse](docs/PostStoryResponse.md)
 - [PreCheckoutQuery](docs/PreCheckoutQuery.md)
 - [PreparedInlineMessage](docs/PreparedInlineMessage.md)
 - [PromoteChatMemberRequest](docs/PromoteChatMemberRequest.md)
 - [PromoteChatMemberResponse](docs/PromoteChatMemberResponse.md)
 - [ProximityAlertTriggered](docs/ProximityAlertTriggered.md)
 - [ReactionCount](docs/ReactionCount.md)
 - [ReactionType](docs/ReactionType.md)
 - [ReactionTypeCustomEmoji](docs/ReactionTypeCustomEmoji.md)
 - [ReactionTypeEmoji](docs/ReactionTypeEmoji.md)
 - [ReactionTypePaid](docs/ReactionTypePaid.md)
 - [ReadBusinessMessageRequest](docs/ReadBusinessMessageRequest.md)
 - [ReadBusinessMessageResponse](docs/ReadBusinessMessageResponse.md)
 - [RefundStarPaymentRequest](docs/RefundStarPaymentRequest.md)
 - [RefundStarPaymentResponse](docs/RefundStarPaymentResponse.md)
 - [RefundedPayment](docs/RefundedPayment.md)
 - [RemoveBusinessAccountProfilePhotoRequest](docs/RemoveBusinessAccountProfilePhotoRequest.md)
 - [RemoveBusinessAccountProfilePhotoResponse](docs/RemoveBusinessAccountProfilePhotoResponse.md)
 - [RemoveChatVerificationRequest](docs/RemoveChatVerificationRequest.md)
 - [RemoveChatVerificationResponse](docs/RemoveChatVerificationResponse.md)
 - [RemoveUserVerificationRequest](docs/RemoveUserVerificationRequest.md)
 - [RemoveUserVerificationResponse](docs/RemoveUserVerificationResponse.md)
 - [ReopenForumTopicRequest](docs/ReopenForumTopicRequest.md)
 - [ReopenForumTopicResponse](docs/ReopenForumTopicResponse.md)
 - [ReopenGeneralForumTopicRequest](docs/ReopenGeneralForumTopicRequest.md)
 - [ReopenGeneralForumTopicResponse](docs/ReopenGeneralForumTopicResponse.md)
 - [ReplaceStickerInSetRequest](docs/ReplaceStickerInSetRequest.md)
 - [ReplaceStickerInSetResponse](docs/ReplaceStickerInSetResponse.md)
 - [ReplyKeyboardMarkup](docs/ReplyKeyboardMarkup.md)
 - [ReplyKeyboardRemove](docs/ReplyKeyboardRemove.md)
 - [ReplyParameters](docs/ReplyParameters.md)
 - [ReplyParametersChatId](docs/ReplyParametersChatId.md)
 - [ResponseParameters](docs/ResponseParameters.md)
 - [RestrictChatMemberRequest](docs/RestrictChatMemberRequest.md)
 - [RestrictChatMemberResponse](docs/RestrictChatMemberResponse.md)
 - [RevenueWithdrawalState](docs/RevenueWithdrawalState.md)
 - [RevenueWithdrawalStateFailed](docs/RevenueWithdrawalStateFailed.md)
 - [RevenueWithdrawalStatePending](docs/RevenueWithdrawalStatePending.md)
 - [RevenueWithdrawalStateSucceeded](docs/RevenueWithdrawalStateSucceeded.md)
 - [RevokeChatInviteLinkRequest](docs/RevokeChatInviteLinkRequest.md)
 - [RevokeChatInviteLinkRequestChatId](docs/RevokeChatInviteLinkRequestChatId.md)
 - [RevokeChatInviteLinkResponse](docs/RevokeChatInviteLinkResponse.md)
 - [SavePreparedInlineMessageRequest](docs/SavePreparedInlineMessageRequest.md)
 - [SavePreparedInlineMessageResponse](docs/SavePreparedInlineMessageResponse.md)
 - [SendAnimationRequest](docs/SendAnimationRequest.md)
 - [SendAnimationResponse](docs/SendAnimationResponse.md)
 - [SendAudioRequest](docs/SendAudioRequest.md)
 - [SendAudioResponse](docs/SendAudioResponse.md)
 - [SendChatActionRequest](docs/SendChatActionRequest.md)
 - [SendChatActionResponse](docs/SendChatActionResponse.md)
 - [SendContactRequest](docs/SendContactRequest.md)
 - [SendContactResponse](docs/SendContactResponse.md)
 - [SendDiceRequest](docs/SendDiceRequest.md)
 - [SendDiceResponse](docs/SendDiceResponse.md)
 - [SendDocumentRequest](docs/SendDocumentRequest.md)
 - [SendDocumentResponse](docs/SendDocumentResponse.md)
 - [SendGameRequest](docs/SendGameRequest.md)
 - [SendGameResponse](docs/SendGameResponse.md)
 - [SendGiftRequest](docs/SendGiftRequest.md)
 - [SendGiftRequestChatId](docs/SendGiftRequestChatId.md)
 - [SendGiftResponse](docs/SendGiftResponse.md)
 - [SendInvoiceRequest](docs/SendInvoiceRequest.md)
 - [SendInvoiceResponse](docs/SendInvoiceResponse.md)
 - [SendLocationRequest](docs/SendLocationRequest.md)
 - [SendLocationResponse](docs/SendLocationResponse.md)
 - [SendMediaGroupRequest](docs/SendMediaGroupRequest.md)
 - [SendMediaGroupRequestMediaInner](docs/SendMediaGroupRequestMediaInner.md)
 - [SendMediaGroupResponse](docs/SendMediaGroupResponse.md)
 - [SendMessageRequest](docs/SendMessageRequest.md)
 - [SendMessageRequestChatId](docs/SendMessageRequestChatId.md)
 - [SendMessageRequestReplyMarkup](docs/SendMessageRequestReplyMarkup.md)
 - [SendMessageResponse](docs/SendMessageResponse.md)
 - [SendPaidMediaRequest](docs/SendPaidMediaRequest.md)
 - [SendPaidMediaRequestChatId](docs/SendPaidMediaRequestChatId.md)
 - [SendPaidMediaResponse](docs/SendPaidMediaResponse.md)
 - [SendPhotoRequest](docs/SendPhotoRequest.md)
 - [SendPhotoResponse](docs/SendPhotoResponse.md)
 - [SendPollRequest](docs/SendPollRequest.md)
 - [SendPollResponse](docs/SendPollResponse.md)
 - [SendStickerRequest](docs/SendStickerRequest.md)
 - [SendStickerResponse](docs/SendStickerResponse.md)
 - [SendVenueRequest](docs/SendVenueRequest.md)
 - [SendVenueResponse](docs/SendVenueResponse.md)
 - [SendVideoNoteRequest](docs/SendVideoNoteRequest.md)
 - [SendVideoNoteResponse](docs/SendVideoNoteResponse.md)
 - [SendVideoRequest](docs/SendVideoRequest.md)
 - [SendVideoResponse](docs/SendVideoResponse.md)
 - [SendVoiceRequest](docs/SendVoiceRequest.md)
 - [SendVoiceResponse](docs/SendVoiceResponse.md)
 - [SentWebAppMessage](docs/SentWebAppMessage.md)
 - [SetBusinessAccountBioRequest](docs/SetBusinessAccountBioRequest.md)
 - [SetBusinessAccountBioResponse](docs/SetBusinessAccountBioResponse.md)
 - [SetBusinessAccountGiftSettingsRequest](docs/SetBusinessAccountGiftSettingsRequest.md)
 - [SetBusinessAccountGiftSettingsResponse](docs/SetBusinessAccountGiftSettingsResponse.md)
 - [SetBusinessAccountNameRequest](docs/SetBusinessAccountNameRequest.md)
 - [SetBusinessAccountNameResponse](docs/SetBusinessAccountNameResponse.md)
 - [SetBusinessAccountProfilePhotoRequest](docs/SetBusinessAccountProfilePhotoRequest.md)
 - [SetBusinessAccountProfilePhotoResponse](docs/SetBusinessAccountProfilePhotoResponse.md)
 - [SetBusinessAccountUsernameRequest](docs/SetBusinessAccountUsernameRequest.md)
 - [SetBusinessAccountUsernameResponse](docs/SetBusinessAccountUsernameResponse.md)
 - [SetChatAdministratorCustomTitleRequest](docs/SetChatAdministratorCustomTitleRequest.md)
 - [SetChatAdministratorCustomTitleResponse](docs/SetChatAdministratorCustomTitleResponse.md)
 - [SetChatDescriptionRequest](docs/SetChatDescriptionRequest.md)
 - [SetChatDescriptionResponse](docs/SetChatDescriptionResponse.md)
 - [SetChatMenuButtonRequest](docs/SetChatMenuButtonRequest.md)
 - [SetChatMenuButtonResponse](docs/SetChatMenuButtonResponse.md)
 - [SetChatPermissionsRequest](docs/SetChatPermissionsRequest.md)
 - [SetChatPermissionsResponse](docs/SetChatPermissionsResponse.md)
 - [SetChatPhotoRequest](docs/SetChatPhotoRequest.md)
 - [SetChatPhotoResponse](docs/SetChatPhotoResponse.md)
 - [SetChatStickerSetRequest](docs/SetChatStickerSetRequest.md)
 - [SetChatStickerSetResponse](docs/SetChatStickerSetResponse.md)
 - [SetChatTitleRequest](docs/SetChatTitleRequest.md)
 - [SetChatTitleResponse](docs/SetChatTitleResponse.md)
 - [SetCustomEmojiStickerSetThumbnailRequest](docs/SetCustomEmojiStickerSetThumbnailRequest.md)
 - [SetCustomEmojiStickerSetThumbnailResponse](docs/SetCustomEmojiStickerSetThumbnailResponse.md)
 - [SetGameScoreRequest](docs/SetGameScoreRequest.md)
 - [SetGameScoreResponse](docs/SetGameScoreResponse.md)
 - [SetMessageReactionRequest](docs/SetMessageReactionRequest.md)
 - [SetMessageReactionResponse](docs/SetMessageReactionResponse.md)
 - [SetMyCommandsRequest](docs/SetMyCommandsRequest.md)
 - [SetMyCommandsResponse](docs/SetMyCommandsResponse.md)
 - [SetMyDefaultAdministratorRightsRequest](docs/SetMyDefaultAdministratorRightsRequest.md)
 - [SetMyDefaultAdministratorRightsResponse](docs/SetMyDefaultAdministratorRightsResponse.md)
 - [SetMyDescriptionRequest](docs/SetMyDescriptionRequest.md)
 - [SetMyDescriptionResponse](docs/SetMyDescriptionResponse.md)
 - [SetMyNameRequest](docs/SetMyNameRequest.md)
 - [SetMyNameResponse](docs/SetMyNameResponse.md)
 - [SetMyShortDescriptionRequest](docs/SetMyShortDescriptionRequest.md)
 - [SetMyShortDescriptionResponse](docs/SetMyShortDescriptionResponse.md)
 - [SetPassportDataErrorsRequest](docs/SetPassportDataErrorsRequest.md)
 - [SetPassportDataErrorsResponse](docs/SetPassportDataErrorsResponse.md)
 - [SetStickerEmojiListRequest](docs/SetStickerEmojiListRequest.md)
 - [SetStickerEmojiListResponse](docs/SetStickerEmojiListResponse.md)
 - [SetStickerKeywordsRequest](docs/SetStickerKeywordsRequest.md)
 - [SetStickerKeywordsResponse](docs/SetStickerKeywordsResponse.md)
 - [SetStickerMaskPositionRequest](docs/SetStickerMaskPositionRequest.md)
 - [SetStickerMaskPositionResponse](docs/SetStickerMaskPositionResponse.md)
 - [SetStickerPositionInSetRequest](docs/SetStickerPositionInSetRequest.md)
 - [SetStickerPositionInSetResponse](docs/SetStickerPositionInSetResponse.md)
 - [SetStickerSetThumbnailRequest](docs/SetStickerSetThumbnailRequest.md)
 - [SetStickerSetThumbnailResponse](docs/SetStickerSetThumbnailResponse.md)
 - [SetStickerSetTitleRequest](docs/SetStickerSetTitleRequest.md)
 - [SetStickerSetTitleResponse](docs/SetStickerSetTitleResponse.md)
 - [SetUserEmojiStatusRequest](docs/SetUserEmojiStatusRequest.md)
 - [SetUserEmojiStatusResponse](docs/SetUserEmojiStatusResponse.md)
 - [SetWebhookRequest](docs/SetWebhookRequest.md)
 - [SetWebhookResponse](docs/SetWebhookResponse.md)
 - [SharedUser](docs/SharedUser.md)
 - [ShippingAddress](docs/ShippingAddress.md)
 - [ShippingOption](docs/ShippingOption.md)
 - [ShippingQuery](docs/ShippingQuery.md)
 - [StarAmount](docs/StarAmount.md)
 - [StarTransaction](docs/StarTransaction.md)
 - [StarTransactions](docs/StarTransactions.md)
 - [Sticker](docs/Sticker.md)
 - [StickerSet](docs/StickerSet.md)
 - [StopMessageLiveLocationRequest](docs/StopMessageLiveLocationRequest.md)
 - [StopMessageLiveLocationResponse](docs/StopMessageLiveLocationResponse.md)
 - [StopPollRequest](docs/StopPollRequest.md)
 - [StopPollResponse](docs/StopPollResponse.md)
 - [Story](docs/Story.md)
 - [StoryArea](docs/StoryArea.md)
 - [StoryAreaPosition](docs/StoryAreaPosition.md)
 - [StoryAreaType](docs/StoryAreaType.md)
 - [StoryAreaTypeLink](docs/StoryAreaTypeLink.md)
 - [StoryAreaTypeLocation](docs/StoryAreaTypeLocation.md)
 - [StoryAreaTypeSuggestedReaction](docs/StoryAreaTypeSuggestedReaction.md)
 - [StoryAreaTypeUniqueGift](docs/StoryAreaTypeUniqueGift.md)
 - [StoryAreaTypeWeather](docs/StoryAreaTypeWeather.md)
 - [SuccessfulPayment](docs/SuccessfulPayment.md)
 - [SwitchInlineQueryChosenChat](docs/SwitchInlineQueryChosenChat.md)
 - [TextQuote](docs/TextQuote.md)
 - [TransactionPartner](docs/TransactionPartner.md)
 - [TransactionPartnerAffiliateProgram](docs/TransactionPartnerAffiliateProgram.md)
 - [TransactionPartnerChat](docs/TransactionPartnerChat.md)
 - [TransactionPartnerFragment](docs/TransactionPartnerFragment.md)
 - [TransactionPartnerOther](docs/TransactionPartnerOther.md)
 - [TransactionPartnerTelegramAds](docs/TransactionPartnerTelegramAds.md)
 - [TransactionPartnerTelegramApi](docs/TransactionPartnerTelegramApi.md)
 - [TransactionPartnerUser](docs/TransactionPartnerUser.md)
 - [TransferBusinessAccountStarsRequest](docs/TransferBusinessAccountStarsRequest.md)
 - [TransferBusinessAccountStarsResponse](docs/TransferBusinessAccountStarsResponse.md)
 - [TransferGiftRequest](docs/TransferGiftRequest.md)
 - [TransferGiftResponse](docs/TransferGiftResponse.md)
 - [UnbanChatMemberRequest](docs/UnbanChatMemberRequest.md)
 - [UnbanChatMemberResponse](docs/UnbanChatMemberResponse.md)
 - [UnbanChatSenderChatRequest](docs/UnbanChatSenderChatRequest.md)
 - [UnbanChatSenderChatResponse](docs/UnbanChatSenderChatResponse.md)
 - [UnhideGeneralForumTopicRequest](docs/UnhideGeneralForumTopicRequest.md)
 - [UnhideGeneralForumTopicResponse](docs/UnhideGeneralForumTopicResponse.md)
 - [UniqueGift](docs/UniqueGift.md)
 - [UniqueGiftBackdrop](docs/UniqueGiftBackdrop.md)
 - [UniqueGiftBackdropColors](docs/UniqueGiftBackdropColors.md)
 - [UniqueGiftInfo](docs/UniqueGiftInfo.md)
 - [UniqueGiftModel](docs/UniqueGiftModel.md)
 - [UniqueGiftSymbol](docs/UniqueGiftSymbol.md)
 - [UnpinAllChatMessagesRequest](docs/UnpinAllChatMessagesRequest.md)
 - [UnpinAllChatMessagesResponse](docs/UnpinAllChatMessagesResponse.md)
 - [UnpinAllForumTopicMessagesRequest](docs/UnpinAllForumTopicMessagesRequest.md)
 - [UnpinAllForumTopicMessagesResponse](docs/UnpinAllForumTopicMessagesResponse.md)
 - [UnpinAllGeneralForumTopicMessagesRequest](docs/UnpinAllGeneralForumTopicMessagesRequest.md)
 - [UnpinAllGeneralForumTopicMessagesResponse](docs/UnpinAllGeneralForumTopicMessagesResponse.md)
 - [UnpinChatMessageRequest](docs/UnpinChatMessageRequest.md)
 - [UnpinChatMessageResponse](docs/UnpinChatMessageResponse.md)
 - [Update](docs/Update.md)
 - [UpgradeGiftRequest](docs/UpgradeGiftRequest.md)
 - [UpgradeGiftResponse](docs/UpgradeGiftResponse.md)
 - [UploadStickerFileRequest](docs/UploadStickerFileRequest.md)
 - [UploadStickerFileResponse](docs/UploadStickerFileResponse.md)
 - [User](docs/User.md)
 - [UserChatBoosts](docs/UserChatBoosts.md)
 - [UserProfilePhotos](docs/UserProfilePhotos.md)
 - [UsersShared](docs/UsersShared.md)
 - [Venue](docs/Venue.md)
 - [VerifyChatRequest](docs/VerifyChatRequest.md)
 - [VerifyChatResponse](docs/VerifyChatResponse.md)
 - [VerifyUserRequest](docs/VerifyUserRequest.md)
 - [VerifyUserResponse](docs/VerifyUserResponse.md)
 - [Video](docs/Video.md)
 - [VideoChatEnded](docs/VideoChatEnded.md)
 - [VideoChatParticipantsInvited](docs/VideoChatParticipantsInvited.md)
 - [VideoChatScheduled](docs/VideoChatScheduled.md)
 - [VideoNote](docs/VideoNote.md)
 - [Voice](docs/Voice.md)
 - [WebAppData](docs/WebAppData.md)
 - [WebAppInfo](docs/WebAppInfo.md)
 - [WebhookInfo](docs/WebhookInfo.md)
 - [WriteAccessAllowed](docs/WriteAccessAllowed.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

gitctrlx@gmail.com

