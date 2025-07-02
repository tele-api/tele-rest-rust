# Rust API client for tele_rest

The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram.
To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.

For more information, please visit [https://github.com/gitctrlx](https://github.com/gitctrlx)

## Overview

- API version: 9.0.0
- Package version: 9.0.0
- Build date: 2025-07-02T07:03:16.715318580Z[Etc/UTC]

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
 - [AffiliateInfo](docs/AffiliateInfo.md)
 - [Animation](docs/Animation.md)
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
 - [Birthdate](docs/Birthdate.md)
 - [BotCommand](docs/BotCommand.md)
 - [BotCommandScope](docs/BotCommandScope.md)
 - [BotCommandScopeAllChatAdministrators](docs/BotCommandScopeAllChatAdministrators.md)
 - [BotCommandScopeAllGroupChats](docs/BotCommandScopeAllGroupChats.md)
 - [BotCommandScopeAllPrivateChats](docs/BotCommandScopeAllPrivateChats.md)
 - [BotCommandScopeChat](docs/BotCommandScopeChat.md)
 - [BotCommandScopeChatAdministrators](docs/BotCommandScopeChatAdministrators.md)
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
 - [Contact](docs/Contact.md)
 - [CopyTextButton](docs/CopyTextButton.md)
 - [Dice](docs/Dice.md)
 - [Document](docs/Document.md)
 - [EncryptedCredentials](docs/EncryptedCredentials.md)
 - [EncryptedPassportElement](docs/EncryptedPassportElement.md)
 - [Error](docs/Error.md)
 - [ExternalReplyInfo](docs/ExternalReplyInfo.md)
 - [File](docs/File.md)
 - [ForceReply](docs/ForceReply.md)
 - [ForumTopic](docs/ForumTopic.md)
 - [ForumTopicCreated](docs/ForumTopicCreated.md)
 - [ForumTopicEdited](docs/ForumTopicEdited.md)
 - [Game](docs/Game.md)
 - [GameHighScore](docs/GameHighScore.md)
 - [Gift](docs/Gift.md)
 - [GiftInfo](docs/GiftInfo.md)
 - [Gifts](docs/Gifts.md)
 - [Giveaway](docs/Giveaway.md)
 - [GiveawayCompleted](docs/GiveawayCompleted.md)
 - [GiveawayCreated](docs/GiveawayCreated.md)
 - [GiveawayWinners](docs/GiveawayWinners.md)
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
 - [LinkPreviewOptions](docs/LinkPreviewOptions.md)
 - [Location](docs/Location.md)
 - [LocationAddress](docs/LocationAddress.md)
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
 - [Poll](docs/Poll.md)
 - [PollAnswer](docs/PollAnswer.md)
 - [PollOption](docs/PollOption.md)
 - [PostAnswerCallbackQueryRequest](docs/PostAnswerCallbackQueryRequest.md)
 - [PostAnswerInlineQueryRequest](docs/PostAnswerInlineQueryRequest.md)
 - [PostAnswerPreCheckoutQueryRequest](docs/PostAnswerPreCheckoutQueryRequest.md)
 - [PostAnswerShippingQueryRequest](docs/PostAnswerShippingQueryRequest.md)
 - [PostAnswerWebAppQuery200Response](docs/PostAnswerWebAppQuery200Response.md)
 - [PostAnswerWebAppQueryRequest](docs/PostAnswerWebAppQueryRequest.md)
 - [PostApproveChatJoinRequestRequest](docs/PostApproveChatJoinRequestRequest.md)
 - [PostBanChatMemberRequest](docs/PostBanChatMemberRequest.md)
 - [PostBanChatMemberRequestChatId](docs/PostBanChatMemberRequestChatId.md)
 - [PostBanChatSenderChatRequest](docs/PostBanChatSenderChatRequest.md)
 - [PostCloseForumTopicRequest](docs/PostCloseForumTopicRequest.md)
 - [PostConvertGiftToStarsRequest](docs/PostConvertGiftToStarsRequest.md)
 - [PostCopyMessage200Response](docs/PostCopyMessage200Response.md)
 - [PostCopyMessageRequest](docs/PostCopyMessageRequest.md)
 - [PostCopyMessagesRequest](docs/PostCopyMessagesRequest.md)
 - [PostCreateChatInviteLink200Response](docs/PostCreateChatInviteLink200Response.md)
 - [PostCreateChatInviteLinkRequest](docs/PostCreateChatInviteLinkRequest.md)
 - [PostCreateChatSubscriptionInviteLinkRequest](docs/PostCreateChatSubscriptionInviteLinkRequest.md)
 - [PostCreateChatSubscriptionInviteLinkRequestChatId](docs/PostCreateChatSubscriptionInviteLinkRequestChatId.md)
 - [PostCreateForumTopic200Response](docs/PostCreateForumTopic200Response.md)
 - [PostCreateForumTopicRequest](docs/PostCreateForumTopicRequest.md)
 - [PostCreateInvoiceLinkRequest](docs/PostCreateInvoiceLinkRequest.md)
 - [PostDeleteBusinessMessagesRequest](docs/PostDeleteBusinessMessagesRequest.md)
 - [PostDeleteChatStickerSetRequest](docs/PostDeleteChatStickerSetRequest.md)
 - [PostDeleteMessageRequest](docs/PostDeleteMessageRequest.md)
 - [PostDeleteMessagesRequest](docs/PostDeleteMessagesRequest.md)
 - [PostDeleteMyCommandsRequest](docs/PostDeleteMyCommandsRequest.md)
 - [PostDeleteStickerFromSetRequest](docs/PostDeleteStickerFromSetRequest.md)
 - [PostDeleteStickerSetRequest](docs/PostDeleteStickerSetRequest.md)
 - [PostDeleteStoryRequest](docs/PostDeleteStoryRequest.md)
 - [PostDeleteWebhookRequest](docs/PostDeleteWebhookRequest.md)
 - [PostEditChatInviteLinkRequest](docs/PostEditChatInviteLinkRequest.md)
 - [PostEditChatSubscriptionInviteLinkRequest](docs/PostEditChatSubscriptionInviteLinkRequest.md)
 - [PostEditForumTopicRequest](docs/PostEditForumTopicRequest.md)
 - [PostEditGeneralForumTopicRequest](docs/PostEditGeneralForumTopicRequest.md)
 - [PostEditMessageCaptionRequest](docs/PostEditMessageCaptionRequest.md)
 - [PostEditMessageLiveLocationRequest](docs/PostEditMessageLiveLocationRequest.md)
 - [PostEditMessageReplyMarkupRequest](docs/PostEditMessageReplyMarkupRequest.md)
 - [PostEditMessageText200Response](docs/PostEditMessageText200Response.md)
 - [PostEditMessageText200ResponseResult](docs/PostEditMessageText200ResponseResult.md)
 - [PostEditMessageTextRequest](docs/PostEditMessageTextRequest.md)
 - [PostEditMessageTextRequestChatId](docs/PostEditMessageTextRequestChatId.md)
 - [PostEditUserStarSubscriptionRequest](docs/PostEditUserStarSubscriptionRequest.md)
 - [PostExportChatInviteLink200Response](docs/PostExportChatInviteLink200Response.md)
 - [PostExportChatInviteLinkRequest](docs/PostExportChatInviteLinkRequest.md)
 - [PostForwardMessageRequest](docs/PostForwardMessageRequest.md)
 - [PostForwardMessageRequestFromChatId](docs/PostForwardMessageRequestFromChatId.md)
 - [PostForwardMessages200Response](docs/PostForwardMessages200Response.md)
 - [PostForwardMessagesRequest](docs/PostForwardMessagesRequest.md)
 - [PostForwardMessagesRequestFromChatId](docs/PostForwardMessagesRequestFromChatId.md)
 - [PostGetAvailableGifts200Response](docs/PostGetAvailableGifts200Response.md)
 - [PostGetBusinessAccountGifts200Response](docs/PostGetBusinessAccountGifts200Response.md)
 - [PostGetBusinessAccountGiftsRequest](docs/PostGetBusinessAccountGiftsRequest.md)
 - [PostGetBusinessAccountStarBalance200Response](docs/PostGetBusinessAccountStarBalance200Response.md)
 - [PostGetBusinessConnection200Response](docs/PostGetBusinessConnection200Response.md)
 - [PostGetBusinessConnectionRequest](docs/PostGetBusinessConnectionRequest.md)
 - [PostGetChat200Response](docs/PostGetChat200Response.md)
 - [PostGetChatAdministrators200Response](docs/PostGetChatAdministrators200Response.md)
 - [PostGetChatMember200Response](docs/PostGetChatMember200Response.md)
 - [PostGetChatMemberCount200Response](docs/PostGetChatMemberCount200Response.md)
 - [PostGetChatMemberRequest](docs/PostGetChatMemberRequest.md)
 - [PostGetChatMenuButton200Response](docs/PostGetChatMenuButton200Response.md)
 - [PostGetChatMenuButtonRequest](docs/PostGetChatMenuButtonRequest.md)
 - [PostGetCustomEmojiStickersRequest](docs/PostGetCustomEmojiStickersRequest.md)
 - [PostGetFile200Response](docs/PostGetFile200Response.md)
 - [PostGetFileRequest](docs/PostGetFileRequest.md)
 - [PostGetForumTopicIconStickers200Response](docs/PostGetForumTopicIconStickers200Response.md)
 - [PostGetGameHighScores200Response](docs/PostGetGameHighScores200Response.md)
 - [PostGetGameHighScoresRequest](docs/PostGetGameHighScoresRequest.md)
 - [PostGetMe200Response](docs/PostGetMe200Response.md)
 - [PostGetMyCommands200Response](docs/PostGetMyCommands200Response.md)
 - [PostGetMyCommandsRequest](docs/PostGetMyCommandsRequest.md)
 - [PostGetMyDefaultAdministratorRights200Response](docs/PostGetMyDefaultAdministratorRights200Response.md)
 - [PostGetMyDefaultAdministratorRightsRequest](docs/PostGetMyDefaultAdministratorRightsRequest.md)
 - [PostGetMyDescription200Response](docs/PostGetMyDescription200Response.md)
 - [PostGetMyName200Response](docs/PostGetMyName200Response.md)
 - [PostGetMyNameRequest](docs/PostGetMyNameRequest.md)
 - [PostGetMyShortDescription200Response](docs/PostGetMyShortDescription200Response.md)
 - [PostGetStarTransactions200Response](docs/PostGetStarTransactions200Response.md)
 - [PostGetStarTransactionsRequest](docs/PostGetStarTransactionsRequest.md)
 - [PostGetStickerSet200Response](docs/PostGetStickerSet200Response.md)
 - [PostGetStickerSetRequest](docs/PostGetStickerSetRequest.md)
 - [PostGetUpdates200Response](docs/PostGetUpdates200Response.md)
 - [PostGetUpdatesRequest](docs/PostGetUpdatesRequest.md)
 - [PostGetUserChatBoosts200Response](docs/PostGetUserChatBoosts200Response.md)
 - [PostGetUserChatBoostsRequest](docs/PostGetUserChatBoostsRequest.md)
 - [PostGetUserChatBoostsRequestChatId](docs/PostGetUserChatBoostsRequestChatId.md)
 - [PostGetUserProfilePhotos200Response](docs/PostGetUserProfilePhotos200Response.md)
 - [PostGetUserProfilePhotosRequest](docs/PostGetUserProfilePhotosRequest.md)
 - [PostGetWebhookInfo200Response](docs/PostGetWebhookInfo200Response.md)
 - [PostGiftPremiumSubscriptionRequest](docs/PostGiftPremiumSubscriptionRequest.md)
 - [PostLeaveChatRequest](docs/PostLeaveChatRequest.md)
 - [PostLeaveChatRequestChatId](docs/PostLeaveChatRequestChatId.md)
 - [PostPinChatMessageRequest](docs/PostPinChatMessageRequest.md)
 - [PostPostStory200Response](docs/PostPostStory200Response.md)
 - [PostPromoteChatMemberRequest](docs/PostPromoteChatMemberRequest.md)
 - [PostReadBusinessMessageRequest](docs/PostReadBusinessMessageRequest.md)
 - [PostRefundStarPaymentRequest](docs/PostRefundStarPaymentRequest.md)
 - [PostRemoveBusinessAccountProfilePhotoRequest](docs/PostRemoveBusinessAccountProfilePhotoRequest.md)
 - [PostRemoveUserVerificationRequest](docs/PostRemoveUserVerificationRequest.md)
 - [PostRestrictChatMemberRequest](docs/PostRestrictChatMemberRequest.md)
 - [PostRestrictChatMemberRequestChatId](docs/PostRestrictChatMemberRequestChatId.md)
 - [PostRevokeChatInviteLinkRequest](docs/PostRevokeChatInviteLinkRequest.md)
 - [PostRevokeChatInviteLinkRequestChatId](docs/PostRevokeChatInviteLinkRequestChatId.md)
 - [PostSavePreparedInlineMessage200Response](docs/PostSavePreparedInlineMessage200Response.md)
 - [PostSavePreparedInlineMessageRequest](docs/PostSavePreparedInlineMessageRequest.md)
 - [PostSendAnimationRequestAnimation](docs/PostSendAnimationRequestAnimation.md)
 - [PostSendAudioRequestAudio](docs/PostSendAudioRequestAudio.md)
 - [PostSendAudioRequestThumbnail](docs/PostSendAudioRequestThumbnail.md)
 - [PostSendChatActionRequest](docs/PostSendChatActionRequest.md)
 - [PostSendContactRequest](docs/PostSendContactRequest.md)
 - [PostSendDiceRequest](docs/PostSendDiceRequest.md)
 - [PostSendDocumentRequestDocument](docs/PostSendDocumentRequestDocument.md)
 - [PostSendGameRequest](docs/PostSendGameRequest.md)
 - [PostSendGiftRequest](docs/PostSendGiftRequest.md)
 - [PostSendGiftRequestChatId](docs/PostSendGiftRequestChatId.md)
 - [PostSendInvoiceRequest](docs/PostSendInvoiceRequest.md)
 - [PostSendLocationRequest](docs/PostSendLocationRequest.md)
 - [PostSendMediaGroup200Response](docs/PostSendMediaGroup200Response.md)
 - [PostSendMediaGroupRequestMediaInner](docs/PostSendMediaGroupRequestMediaInner.md)
 - [PostSendMessage200Response](docs/PostSendMessage200Response.md)
 - [PostSendMessageRequest](docs/PostSendMessageRequest.md)
 - [PostSendMessageRequestChatId](docs/PostSendMessageRequestChatId.md)
 - [PostSendMessageRequestReplyMarkup](docs/PostSendMessageRequestReplyMarkup.md)
 - [PostSendPaidMediaRequestChatId](docs/PostSendPaidMediaRequestChatId.md)
 - [PostSendPhotoRequestPhoto](docs/PostSendPhotoRequestPhoto.md)
 - [PostSendPollRequest](docs/PostSendPollRequest.md)
 - [PostSendStickerRequestSticker](docs/PostSendStickerRequestSticker.md)
 - [PostSendVenueRequest](docs/PostSendVenueRequest.md)
 - [PostSendVideoNoteRequestVideoNote](docs/PostSendVideoNoteRequestVideoNote.md)
 - [PostSendVideoRequestCover](docs/PostSendVideoRequestCover.md)
 - [PostSendVideoRequestVideo](docs/PostSendVideoRequestVideo.md)
 - [PostSendVoiceRequestVoice](docs/PostSendVoiceRequestVoice.md)
 - [PostSetBusinessAccountBioRequest](docs/PostSetBusinessAccountBioRequest.md)
 - [PostSetBusinessAccountGiftSettingsRequest](docs/PostSetBusinessAccountGiftSettingsRequest.md)
 - [PostSetBusinessAccountNameRequest](docs/PostSetBusinessAccountNameRequest.md)
 - [PostSetBusinessAccountUsernameRequest](docs/PostSetBusinessAccountUsernameRequest.md)
 - [PostSetChatAdministratorCustomTitleRequest](docs/PostSetChatAdministratorCustomTitleRequest.md)
 - [PostSetChatDescriptionRequest](docs/PostSetChatDescriptionRequest.md)
 - [PostSetChatMenuButtonRequest](docs/PostSetChatMenuButtonRequest.md)
 - [PostSetChatPermissionsRequest](docs/PostSetChatPermissionsRequest.md)
 - [PostSetChatStickerSetRequest](docs/PostSetChatStickerSetRequest.md)
 - [PostSetChatTitleRequest](docs/PostSetChatTitleRequest.md)
 - [PostSetCustomEmojiStickerSetThumbnailRequest](docs/PostSetCustomEmojiStickerSetThumbnailRequest.md)
 - [PostSetGameScoreRequest](docs/PostSetGameScoreRequest.md)
 - [PostSetMessageReactionRequest](docs/PostSetMessageReactionRequest.md)
 - [PostSetMyCommandsRequest](docs/PostSetMyCommandsRequest.md)
 - [PostSetMyDefaultAdministratorRightsRequest](docs/PostSetMyDefaultAdministratorRightsRequest.md)
 - [PostSetMyDescriptionRequest](docs/PostSetMyDescriptionRequest.md)
 - [PostSetMyNameRequest](docs/PostSetMyNameRequest.md)
 - [PostSetMyShortDescriptionRequest](docs/PostSetMyShortDescriptionRequest.md)
 - [PostSetPassportDataErrorsRequest](docs/PostSetPassportDataErrorsRequest.md)
 - [PostSetStickerEmojiListRequest](docs/PostSetStickerEmojiListRequest.md)
 - [PostSetStickerKeywordsRequest](docs/PostSetStickerKeywordsRequest.md)
 - [PostSetStickerMaskPositionRequest](docs/PostSetStickerMaskPositionRequest.md)
 - [PostSetStickerPositionInSetRequest](docs/PostSetStickerPositionInSetRequest.md)
 - [PostSetStickerSetThumbnailRequestThumbnail](docs/PostSetStickerSetThumbnailRequestThumbnail.md)
 - [PostSetStickerSetTitleRequest](docs/PostSetStickerSetTitleRequest.md)
 - [PostSetUserEmojiStatusRequest](docs/PostSetUserEmojiStatusRequest.md)
 - [PostSetWebhook200Response](docs/PostSetWebhook200Response.md)
 - [PostStopMessageLiveLocationRequest](docs/PostStopMessageLiveLocationRequest.md)
 - [PostStopPoll200Response](docs/PostStopPoll200Response.md)
 - [PostStopPollRequest](docs/PostStopPollRequest.md)
 - [PostTransferBusinessAccountStarsRequest](docs/PostTransferBusinessAccountStarsRequest.md)
 - [PostTransferGiftRequest](docs/PostTransferGiftRequest.md)
 - [PostUnbanChatMemberRequest](docs/PostUnbanChatMemberRequest.md)
 - [PostUnpinChatMessageRequest](docs/PostUnpinChatMessageRequest.md)
 - [PostUpgradeGiftRequest](docs/PostUpgradeGiftRequest.md)
 - [PostVerifyChatRequest](docs/PostVerifyChatRequest.md)
 - [PostVerifyUserRequest](docs/PostVerifyUserRequest.md)
 - [PreCheckoutQuery](docs/PreCheckoutQuery.md)
 - [PreparedInlineMessage](docs/PreparedInlineMessage.md)
 - [ProximityAlertTriggered](docs/ProximityAlertTriggered.md)
 - [ReactionCount](docs/ReactionCount.md)
 - [ReactionType](docs/ReactionType.md)
 - [ReactionTypeCustomEmoji](docs/ReactionTypeCustomEmoji.md)
 - [ReactionTypeEmoji](docs/ReactionTypeEmoji.md)
 - [ReactionTypePaid](docs/ReactionTypePaid.md)
 - [RefundedPayment](docs/RefundedPayment.md)
 - [ReplyKeyboardMarkup](docs/ReplyKeyboardMarkup.md)
 - [ReplyKeyboardRemove](docs/ReplyKeyboardRemove.md)
 - [ReplyParameters](docs/ReplyParameters.md)
 - [ReplyParametersChatId](docs/ReplyParametersChatId.md)
 - [ResponseParameters](docs/ResponseParameters.md)
 - [RevenueWithdrawalState](docs/RevenueWithdrawalState.md)
 - [RevenueWithdrawalStateFailed](docs/RevenueWithdrawalStateFailed.md)
 - [RevenueWithdrawalStatePending](docs/RevenueWithdrawalStatePending.md)
 - [RevenueWithdrawalStateSucceeded](docs/RevenueWithdrawalStateSucceeded.md)
 - [SentWebAppMessage](docs/SentWebAppMessage.md)
 - [SharedUser](docs/SharedUser.md)
 - [ShippingAddress](docs/ShippingAddress.md)
 - [ShippingOption](docs/ShippingOption.md)
 - [ShippingQuery](docs/ShippingQuery.md)
 - [StarAmount](docs/StarAmount.md)
 - [StarTransaction](docs/StarTransaction.md)
 - [StarTransactions](docs/StarTransactions.md)
 - [Sticker](docs/Sticker.md)
 - [StickerSet](docs/StickerSet.md)
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
 - [UniqueGift](docs/UniqueGift.md)
 - [UniqueGiftBackdrop](docs/UniqueGiftBackdrop.md)
 - [UniqueGiftBackdropColors](docs/UniqueGiftBackdropColors.md)
 - [UniqueGiftInfo](docs/UniqueGiftInfo.md)
 - [UniqueGiftModel](docs/UniqueGiftModel.md)
 - [UniqueGiftSymbol](docs/UniqueGiftSymbol.md)
 - [Update](docs/Update.md)
 - [User](docs/User.md)
 - [UserChatBoosts](docs/UserChatBoosts.md)
 - [UserProfilePhotos](docs/UserProfilePhotos.md)
 - [UsersShared](docs/UsersShared.md)
 - [Venue](docs/Venue.md)
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

