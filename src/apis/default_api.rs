//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-02T09:17:04.370667370Z[Etc/UTC]
//! - **Generator Version**: 7.14.0
//!
//! <details>
//! <summary><strong>⚠️ Important Disclaimer & Limitation of Liability</strong></summary>
//! <br>
//! > **IMPORTANT**: This software is provided "as is" without any warranties, express or implied, including but not limited
//! > to warranties of merchantability, fitness for a particular purpose, or non-infringement. The developers, contributors,
//! > and licensors (collectively, "Developers") make no representations regarding the accuracy, completeness, or reliability
//! > of this software or its outputs.
//! >
//! > This client is not intended to provide financial, investment, tax, or legal advice. It facilitates interaction with the
//! > Telegram Bot API service but does not endorse or recommend any financial actions, including the purchase, sale, or holding of
//! > financial instruments (e.g., stocks, bonds, derivatives, cryptocurrencies). Users must consult qualified financial or
//! > legal professionals before making decisions based on this software's outputs.
//! >
//! > Financial markets are inherently speculative and carry significant risks. Using this software in trading, analysis, or
//! > other financial activities may result in substantial losses, including total loss of capital. The Developers are not
//! > liable for any losses or damages arising from such use. Users assume full responsibility for validating the software's
//! > outputs and ensuring their suitability for intended purposes.
//! >
//! > This client may rely on third-party data or services (e.g., market feeds, APIs). The Developers do not control or verify
//! > the accuracy of these services and are not liable for any errors, delays, or losses resulting from their use. Users must
//! > comply with third-party terms and conditions.
//! >
//! > Users are solely responsible for ensuring compliance with all applicable financial, tax, and regulatory requirements in
//! > their jurisdiction. This includes obtaining necessary licenses or approvals for trading or investment activities. The
//! > Developers disclaim liability for any legal consequences arising from non-compliance.
//! >
//! > To the fullest extent permitted by law, the Developers shall not be liable for any direct, indirect, incidental,
//! > consequential, or punitive damages arising from the use or inability to use this software, including but not limited to
//! > loss of profits, data, or business opportunities.
//!
//! </details>

use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
use crate::apis::ContentType;

#[async_trait]
pub trait DefaultApi: Send + Sync {

    /// POST /addStickerToSet
    ///
    /// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.
    async fn post_add_sticker_to_set<'user_id, 'name, 'sticker>(&self, user_id: i32, name: &'name str, sticker: models::models::InputSticker) -> Result<ResponseContent<PostAddStickerToSetSuccess>, Error<PostAddStickerToSetError>>;

    /// POST /answerCallbackQuery
    ///
    /// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.  Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    async fn post_answer_callback_query<'callback_query_id, 'text, 'show_alert, 'url, 'cache_time>(&self, callback_query_id: &'callback_query_id str, text: Option<&'text str>, show_alert: Option<bool>, url: Option<&'url str>, cache_time: Option<i32>) -> Result<ResponseContent<PostAnswerCallbackQuerySuccess>, Error<PostAnswerCallbackQueryError>>;

    /// POST /answerInlineQuery
    ///
    /// Use this method to send answers to an inline query. On success, *True* is returned.   No more than **50** results per query are allowed.
    async fn post_answer_inline_query<'inline_query_id, 'results, 'cache_time, 'is_personal, 'next_offset, 'button>(&self, inline_query_id: &'inline_query_id str, results: Vec<models::InlineQueryResult>, cache_time: Option<i32>, is_personal: Option<bool>, next_offset: Option<&'next_offset str>, button: Option<models::models::InlineQueryResultsButton>) -> Result<ResponseContent<PostAnswerInlineQuerySuccess>, Error<PostAnswerInlineQueryError>>;

    /// POST /answerPreCheckoutQuery
    ///
    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\\_checkout\\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    async fn post_answer_pre_checkout_query<'pre_checkout_query_id, 'ok, 'error_message>(&self, pre_checkout_query_id: &'pre_checkout_query_id str, ok: bool, error_message: Option<&'error_message str>) -> Result<ResponseContent<PostAnswerPreCheckoutQuerySuccess>, Error<PostAnswerPreCheckoutQueryError>>;

    /// POST /answerShippingQuery
    ///
    /// If you sent an invoice requesting a shipping address and the parameter *is\\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
    async fn post_answer_shipping_query<'shipping_query_id, 'ok, 'shipping_options, 'error_message>(&self, shipping_query_id: &'shipping_query_id str, ok: bool, shipping_options: Option<Vec<models::ShippingOption>>, error_message: Option<&'error_message str>) -> Result<ResponseContent<PostAnswerShippingQuerySuccess>, Error<PostAnswerShippingQueryError>>;

    /// POST /answerWebAppQuery
    ///
    /// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
    async fn post_answer_web_app_query<'web_app_query_id, 'result>(&self, web_app_query_id: &'web_app_query_id str, result: models::models::InlineQueryResult) -> Result<ResponseContent<PostAnswerWebAppQuerySuccess>, Error<PostAnswerWebAppQueryError>>;

    /// POST /approveChatJoinRequest
    ///
    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn post_approve_chat_join_request<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessageRequestChatId, user_id: i32) -> Result<ResponseContent<PostApproveChatJoinRequestSuccess>, Error<PostApproveChatJoinRequestError>>;

    /// POST /banChatMember
    ///
    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_ban_chat_member<'chat_id, 'user_id, 'until_date, 'revoke_messages>(&self, chat_id: models::models::BanChatMemberRequestChatId, user_id: i32, until_date: Option<i32>, revoke_messages: Option<bool>) -> Result<ResponseContent<PostBanChatMemberSuccess>, Error<PostBanChatMemberError>>;

    /// POST /banChatSenderChat
    ///
    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_ban_chat_sender_chat<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessageRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<PostBanChatSenderChatSuccess>, Error<PostBanChatSenderChatError>>;

    /// POST /close
    ///
    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
    async fn post_close<>(&self, ) -> Result<ResponseContent<PostCloseSuccess>, Error<PostCloseError>>;

    /// POST /closeForumTopic
    ///
    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn post_close_forum_topic<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostCloseForumTopicSuccess>, Error<PostCloseForumTopicError>>;

    /// POST /closeGeneralForumTopic
    ///
    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn post_close_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostCloseGeneralForumTopicSuccess>, Error<PostCloseGeneralForumTopicError>>;

    /// POST /convertGiftToStars
    ///
    /// Converts a given regular gift to Telegram Stars. Requires the *can\\_convert\\_gifts\\_to\\_stars* business bot right. Returns *True* on success.
    async fn post_convert_gift_to_stars<'business_connection_id, 'owned_gift_id>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str) -> Result<ResponseContent<PostConvertGiftToStarsSuccess>, Error<PostConvertGiftToStarsError>>;

    /// POST /copyMessage
    ///
    /// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api/#forwardmessage), but the copied message doesn't have a link to the original message. Returns the [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent message on success.
    async fn post_copy_message<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessageRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostCopyMessageSuccess>, Error<PostCopyMessageError>>;

    /// POST /copyMessages
    ///
    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn post_copy_messages<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content, 'remove_caption>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessagesRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, remove_caption: Option<bool>) -> Result<ResponseContent<PostCopyMessagesSuccess>, Error<PostCopyMessagesError>>;

    /// POST /createChatInviteLink
    ///
    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_create_chat_invite_link<'chat_id, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessageRequestChatId, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<PostCreateChatInviteLinkSuccess>, Error<PostCreateChatInviteLinkError>>;

    /// POST /createChatSubscriptionInviteLink
    ///
    /// Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\\_invite\\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_create_chat_subscription_invite_link<'chat_id, 'subscription_period, 'subscription_price, 'name>(&self, chat_id: models::models::CreateChatSubscriptionInviteLinkRequestChatId, subscription_period: i32, subscription_price: i32, name: Option<&'name str>) -> Result<ResponseContent<PostCreateChatSubscriptionInviteLinkSuccess>, Error<PostCreateChatSubscriptionInviteLinkError>>;

    /// POST /createForumTopic
    ///
    /// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
    async fn post_create_forum_topic<'chat_id, 'name, 'icon_color, 'icon_custom_emoji_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, name: &'name str, icon_color: Option<i32>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<PostCreateForumTopicSuccess>, Error<PostCreateForumTopicError>>;

    /// POST /createInvoiceLink
    ///
    /// Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
    async fn post_create_invoice_link<'title, 'description, 'payload, 'currency, 'prices, 'business_connection_id, 'provider_token, 'subscription_period, 'max_tip_amount, 'suggested_tip_amounts, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible>(&self, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, business_connection_id: Option<&'business_connection_id str>, provider_token: Option<&'provider_token str>, subscription_period: Option<i32>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>) -> Result<ResponseContent<PostCreateInvoiceLinkSuccess>, Error<PostCreateInvoiceLinkError>>;

    /// POST /createNewStickerSet
    ///
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
    async fn post_create_new_sticker_set<'user_id, 'name, 'title, 'stickers, 'sticker_type, 'needs_repainting>(&self, user_id: i32, name: &'name str, title: &'title str, stickers: Vec<models::InputSticker>, sticker_type: Option<&'sticker_type str>, needs_repainting: Option<bool>) -> Result<ResponseContent<PostCreateNewStickerSetSuccess>, Error<PostCreateNewStickerSetError>>;

    /// POST /declineChatJoinRequest
    ///
    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn post_decline_chat_join_request<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessageRequestChatId, user_id: i32) -> Result<ResponseContent<PostDeclineChatJoinRequestSuccess>, Error<PostDeclineChatJoinRequestError>>;

    /// POST /deleteBusinessMessages
    ///
    /// Delete messages on behalf of a business account. Requires the *can\\_delete\\_outgoing\\_messages* business bot right to delete messages sent by the bot itself, or the *can\\_delete\\_all\\_messages* business bot right to delete any message. Returns *True* on success.
    async fn post_delete_business_messages<'business_connection_id, 'message_ids>(&self, business_connection_id: &'business_connection_id str, message_ids: Vec<i32>) -> Result<ResponseContent<PostDeleteBusinessMessagesSuccess>, Error<PostDeleteBusinessMessagesError>>;

    /// POST /deleteChatPhoto
    ///
    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_delete_chat_photo<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostDeleteChatPhotoSuccess>, Error<PostDeleteChatPhotoError>>;

    /// POST /deleteChatStickerSet
    ///
    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn post_delete_chat_sticker_set<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostDeleteChatStickerSetSuccess>, Error<PostDeleteChatStickerSetError>>;

    /// POST /deleteForumTopic
    ///
    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_delete\\_messages* administrator rights. Returns *True* on success.
    async fn post_delete_forum_topic<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostDeleteForumTopicSuccess>, Error<PostDeleteForumTopicError>>;

    /// POST /deleteMessage
    ///
    /// Use this method to delete a message, including service messages, with the following limitations:   \\- A message can only be deleted if it was sent less than 48 hours ago.   \\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.   \\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.   \\- Bots can delete outgoing messages in private chats, groups, and supergroups.   \\- Bots can delete incoming messages in private chats.   \\- Bots granted *can\\_post\\_messages* permissions can delete outgoing messages in channels.   \\- If the bot is an administrator of a group, it can delete any message there.   \\- If the bot has *can\\_delete\\_messages* permission in a supergroup or a channel, it can delete any message there.   Returns *True* on success.
    async fn post_delete_message<'chat_id, 'message_id>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32) -> Result<ResponseContent<PostDeleteMessageSuccess>, Error<PostDeleteMessageError>>;

    /// POST /deleteMessages
    ///
    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
    async fn post_delete_messages<'chat_id, 'message_ids>(&self, chat_id: models::models::SendMessageRequestChatId, message_ids: Vec<i32>) -> Result<ResponseContent<PostDeleteMessagesSuccess>, Error<PostDeleteMessagesError>>;

    /// POST /deleteMyCommands
    ///
    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
    async fn post_delete_my_commands<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostDeleteMyCommandsSuccess>, Error<PostDeleteMyCommandsError>>;

    /// POST /deleteStickerFromSet
    ///
    /// Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
    async fn post_delete_sticker_from_set<'sticker>(&self, sticker: &'sticker str) -> Result<ResponseContent<PostDeleteStickerFromSetSuccess>, Error<PostDeleteStickerFromSetError>>;

    /// POST /deleteStickerSet
    ///
    /// Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
    async fn post_delete_sticker_set<'name>(&self, name: &'name str) -> Result<ResponseContent<PostDeleteStickerSetSuccess>, Error<PostDeleteStickerSetError>>;

    /// POST /deleteStory
    ///
    /// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns *True* on success.
    async fn post_delete_story<'business_connection_id, 'story_id>(&self, business_connection_id: &'business_connection_id str, story_id: i32) -> Result<ResponseContent<PostDeleteStorySuccess>, Error<PostDeleteStoryError>>;

    /// POST /deleteWebhook
    ///
    /// Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
    async fn post_delete_webhook<'drop_pending_updates>(&self, drop_pending_updates: Option<bool>) -> Result<ResponseContent<PostDeleteWebhookSuccess>, Error<PostDeleteWebhookError>>;

    /// POST /editChatInviteLink
    ///
    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_edit_chat_invite_link<'chat_id, 'invite_link, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessageRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<PostEditChatInviteLinkSuccess>, Error<PostEditChatInviteLinkError>>;

    /// POST /editChatSubscriptionInviteLink
    ///
    /// Use this method to edit a subscription invite link created by the bot. The bot must have the *can\\_invite\\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_edit_chat_subscription_invite_link<'chat_id, 'invite_link, 'name>(&self, chat_id: models::models::SendMessageRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>) -> Result<ResponseContent<PostEditChatSubscriptionInviteLinkSuccess>, Error<PostEditChatSubscriptionInviteLinkError>>;

    /// POST /editForumTopic
    ///
    /// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn post_edit_forum_topic<'chat_id, 'message_thread_id, 'name, 'icon_custom_emoji_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32, name: Option<&'name str>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<PostEditForumTopicSuccess>, Error<PostEditForumTopicError>>;

    /// POST /editGeneralForumTopic
    ///
    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn post_edit_general_forum_topic<'chat_id, 'name>(&self, chat_id: models::models::BotCommandScopeChatChatId, name: &'name str) -> Result<ResponseContent<PostEditGeneralForumTopicSuccess>, Error<PostEditGeneralForumTopicError>>;

    /// POST /editMessageCaption
    ///
    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_caption<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageCaptionSuccess>, Error<PostEditMessageCaptionError>>;

    /// POST /editMessageLiveLocation
    ///
    /// Use this method to edit live location messages. A location can be edited until its *live\\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn post_edit_message_live_location<'latitude, 'longitude, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'live_period, 'horizontal_accuracy, 'heading, 'proximity_alert_radius, 'reply_markup>(&self, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, live_period: Option<i32>, horizontal_accuracy: Option<f64>, heading: Option<i32>, proximity_alert_radius: Option<i32>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageLiveLocationSuccess>, Error<PostEditMessageLiveLocationError>>;

    /// POST /editMessageMedia
    ///
    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_media<'media, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, media: models::models::InputMedia, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageMediaSuccess>, Error<PostEditMessageMediaError>>;

    /// POST /editMessageReplyMarkup
    ///
    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_reply_markup<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageReplyMarkupSuccess>, Error<PostEditMessageReplyMarkupError>>;

    /// POST /editMessageText
    ///
    /// Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_text<'text, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'parse_mode, 'entities, 'link_preview_options, 'reply_markup>(&self, text: &'text str, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageTextSuccess>, Error<PostEditMessageTextError>>;

    /// POST /editStory
    ///
    /// Edits a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn post_edit_story<'business_connection_id, 'story_id, 'content, 'caption, 'parse_mode, 'caption_entities, 'areas>(&self, business_connection_id: &'business_connection_id str, story_id: i32, content: models::models::InputStoryContent, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>) -> Result<ResponseContent<PostEditStorySuccess>, Error<PostEditStoryError>>;

    /// POST /editUserStarSubscription
    ///
    /// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
    async fn post_edit_user_star_subscription<'user_id, 'telegram_payment_charge_id, 'is_canceled>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str, is_canceled: bool) -> Result<ResponseContent<PostEditUserStarSubscriptionSuccess>, Error<PostEditUserStarSubscriptionError>>;

    /// POST /exportChatInviteLink
    ///
    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
    async fn post_export_chat_invite_link<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostExportChatInviteLinkSuccess>, Error<PostExportChatInviteLinkError>>;

    /// POST /forwardMessage
    ///
    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_forward_message<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessageRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostForwardMessageSuccess>, Error<PostForwardMessageError>>;

    /// POST /forwardMessages
    ///
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn post_forward_messages<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessagesRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostForwardMessagesSuccess>, Error<PostForwardMessagesError>>;

    /// POST /getAvailableGifts
    ///
    /// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
    async fn post_get_available_gifts<>(&self, ) -> Result<ResponseContent<PostGetAvailableGiftsSuccess>, Error<PostGetAvailableGiftsError>>;

    /// POST /getBusinessAccountGifts
    ///
    /// Returns the gifts received and owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [OwnedGifts](https://core.telegram.org/bots/api/#ownedgifts) on success.
    async fn post_get_business_account_gifts<'business_connection_id, 'exclude_unsaved, 'exclude_saved, 'exclude_unlimited, 'exclude_limited, 'exclude_unique, 'sort_by_price, 'offset, 'limit>(&self, business_connection_id: &'business_connection_id str, exclude_unsaved: Option<bool>, exclude_saved: Option<bool>, exclude_unlimited: Option<bool>, exclude_limited: Option<bool>, exclude_unique: Option<bool>, sort_by_price: Option<bool>, offset: Option<&'offset str>, limit: Option<i32>) -> Result<ResponseContent<PostGetBusinessAccountGiftsSuccess>, Error<PostGetBusinessAccountGiftsError>>;

    /// POST /getBusinessAccountStarBalance
    ///
    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [StarAmount](https://core.telegram.org/bots/api/#staramount) on success.
    async fn post_get_business_account_star_balance<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<PostGetBusinessAccountStarBalanceSuccess>, Error<PostGetBusinessAccountStarBalanceError>>;

    /// POST /getBusinessConnection
    ///
    /// Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
    async fn post_get_business_connection<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<PostGetBusinessConnectionSuccess>, Error<PostGetBusinessConnectionError>>;

    /// POST /getChat
    ///
    /// Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
    async fn post_get_chat<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostGetChatSuccess>, Error<PostGetChatError>>;

    /// POST /getChatAdministrators
    ///
    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
    async fn post_get_chat_administrators<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostGetChatAdministratorsSuccess>, Error<PostGetChatAdministratorsError>>;

    /// POST /getChatMember
    ///
    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
    async fn post_get_chat_member<'chat_id, 'user_id>(&self, chat_id: models::models::LeaveChatRequestChatId, user_id: i32) -> Result<ResponseContent<PostGetChatMemberSuccess>, Error<PostGetChatMemberError>>;

    /// POST /getChatMemberCount
    ///
    /// Use this method to get the number of members in a chat. Returns *Int* on success.
    async fn post_get_chat_member_count<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostGetChatMemberCountSuccess>, Error<PostGetChatMemberCountError>>;

    /// POST /getChatMenuButton
    ///
    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
    async fn post_get_chat_menu_button<'chat_id>(&self, chat_id: Option<i32>) -> Result<ResponseContent<PostGetChatMenuButtonSuccess>, Error<PostGetChatMenuButtonError>>;

    /// POST /getCustomEmojiStickers
    ///
    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn post_get_custom_emoji_stickers<'custom_emoji_ids>(&self, custom_emoji_ids: Vec<String>) -> Result<ResponseContent<PostGetCustomEmojiStickersSuccess>, Error<PostGetCustomEmojiStickersError>>;

    /// POST /getFile
    ///
    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
    async fn post_get_file<'file_id>(&self, file_id: &'file_id str) -> Result<ResponseContent<PostGetFileSuccess>, Error<PostGetFileError>>;

    /// POST /getForumTopicIconStickers
    ///
    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn post_get_forum_topic_icon_stickers<>(&self, ) -> Result<ResponseContent<PostGetForumTopicIconStickersSuccess>, Error<PostGetForumTopicIconStickersError>>;

    /// POST /getGameHighScores
    ///
    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.  This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
    async fn post_get_game_high_scores<'user_id, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<PostGetGameHighScoresSuccess>, Error<PostGetGameHighScoresError>>;

    /// POST /getMe
    ///
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
    async fn post_get_me<>(&self, ) -> Result<ResponseContent<PostGetMeSuccess>, Error<PostGetMeError>>;

    /// POST /getMyCommands
    ///
    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
    async fn post_get_my_commands<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyCommandsSuccess>, Error<PostGetMyCommandsError>>;

    /// POST /getMyDefaultAdministratorRights
    ///
    /// Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
    async fn post_get_my_default_administrator_rights<'for_channels>(&self, for_channels: Option<bool>) -> Result<ResponseContent<PostGetMyDefaultAdministratorRightsSuccess>, Error<PostGetMyDefaultAdministratorRightsError>>;

    /// POST /getMyDescription
    ///
    /// Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
    async fn post_get_my_description<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyDescriptionSuccess>, Error<PostGetMyDescriptionError>>;

    /// POST /getMyName
    ///
    /// Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
    async fn post_get_my_name<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyNameSuccess>, Error<PostGetMyNameError>>;

    /// POST /getMyShortDescription
    ///
    /// Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
    async fn post_get_my_short_description<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyShortDescriptionSuccess>, Error<PostGetMyShortDescriptionError>>;

    /// POST /getStarTransactions
    ///
    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
    async fn post_get_star_transactions<'offset, 'limit>(&self, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<PostGetStarTransactionsSuccess>, Error<PostGetStarTransactionsError>>;

    /// POST /getStickerSet
    ///
    /// Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
    async fn post_get_sticker_set<'name>(&self, name: &'name str) -> Result<ResponseContent<PostGetStickerSetSuccess>, Error<PostGetStickerSetError>>;

    /// POST /getUpdates
    ///
    /// Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.
    async fn post_get_updates<'offset, 'limit, 'timeout, 'allowed_updates>(&self, offset: Option<i32>, limit: Option<i32>, timeout: Option<i32>, allowed_updates: Option<Vec<String>>) -> Result<ResponseContent<PostGetUpdatesSuccess>, Error<PostGetUpdatesError>>;

    /// POST /getUserChatBoosts
    ///
    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
    async fn post_get_user_chat_boosts<'chat_id, 'user_id>(&self, chat_id: models::models::GetUserChatBoostsRequestChatId, user_id: i32) -> Result<ResponseContent<PostGetUserChatBoostsSuccess>, Error<PostGetUserChatBoostsError>>;

    /// POST /getUserProfilePhotos
    ///
    /// Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
    async fn post_get_user_profile_photos<'user_id, 'offset, 'limit>(&self, user_id: i32, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<PostGetUserProfilePhotosSuccess>, Error<PostGetUserProfilePhotosError>>;

    /// POST /getWebhookInfo
    ///
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
    async fn post_get_webhook_info<>(&self, ) -> Result<ResponseContent<PostGetWebhookInfoSuccess>, Error<PostGetWebhookInfoError>>;

    /// POST /giftPremiumSubscription
    ///
    /// Gifts a Telegram Premium subscription to the given user. Returns *True* on success.
    async fn post_gift_premium_subscription<'user_id, 'month_count, 'star_count, 'text, 'text_parse_mode, 'text_entities>(&self, user_id: i32, month_count: i32, star_count: i32, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<PostGiftPremiumSubscriptionSuccess>, Error<PostGiftPremiumSubscriptionError>>;

    /// POST /hideGeneralForumTopic
    ///
    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
    async fn post_hide_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostHideGeneralForumTopicSuccess>, Error<PostHideGeneralForumTopicError>>;

    /// POST /leaveChat
    ///
    /// Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    async fn post_leave_chat<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostLeaveChatSuccess>, Error<PostLeaveChatError>>;

    /// POST /logOut
    ///
    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
    async fn post_log_out<>(&self, ) -> Result<ResponseContent<PostLogOutSuccess>, Error<PostLogOutError>>;

    /// POST /pinChatMessage
    ///
    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn post_pin_chat_message<'chat_id, 'message_id, 'business_connection_id, 'disable_notification>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, disable_notification: Option<bool>) -> Result<ResponseContent<PostPinChatMessageSuccess>, Error<PostPinChatMessageError>>;

    /// POST /postStory
    ///
    /// Posts a story on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn post_post_story<'business_connection_id, 'content, 'active_period, 'caption, 'parse_mode, 'caption_entities, 'areas, 'post_to_chat_page, 'protect_content>(&self, business_connection_id: &'business_connection_id str, content: models::models::InputStoryContent, active_period: i32, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>, post_to_chat_page: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostPostStorySuccess>, Error<PostPostStoryError>>;

    /// POST /promoteChatMember
    ///
    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    async fn post_promote_chat_member<'chat_id, 'user_id, 'is_anonymous, 'can_manage_chat, 'can_delete_messages, 'can_manage_video_chats, 'can_restrict_members, 'can_promote_members, 'can_change_info, 'can_invite_users, 'can_post_stories, 'can_edit_stories, 'can_delete_stories, 'can_post_messages, 'can_edit_messages, 'can_pin_messages, 'can_manage_topics>(&self, chat_id: models::models::SendMessageRequestChatId, user_id: i32, is_anonymous: Option<bool>, can_manage_chat: Option<bool>, can_delete_messages: Option<bool>, can_manage_video_chats: Option<bool>, can_restrict_members: Option<bool>, can_promote_members: Option<bool>, can_change_info: Option<bool>, can_invite_users: Option<bool>, can_post_stories: Option<bool>, can_edit_stories: Option<bool>, can_delete_stories: Option<bool>, can_post_messages: Option<bool>, can_edit_messages: Option<bool>, can_pin_messages: Option<bool>, can_manage_topics: Option<bool>) -> Result<ResponseContent<PostPromoteChatMemberSuccess>, Error<PostPromoteChatMemberError>>;

    /// POST /readBusinessMessage
    ///
    /// Marks incoming message as read on behalf of a business account. Requires the *can\\_read\\_messages* business bot right. Returns *True* on success.
    async fn post_read_business_message<'business_connection_id, 'chat_id, 'message_id>(&self, business_connection_id: &'business_connection_id str, chat_id: i32, message_id: i32) -> Result<ResponseContent<PostReadBusinessMessageSuccess>, Error<PostReadBusinessMessageError>>;

    /// POST /refundStarPayment
    ///
    /// Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
    async fn post_refund_star_payment<'user_id, 'telegram_payment_charge_id>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str) -> Result<ResponseContent<PostRefundStarPaymentSuccess>, Error<PostRefundStarPaymentError>>;

    /// POST /removeBusinessAccountProfilePhoto
    ///
    /// Removes the current profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn post_remove_business_account_profile_photo<'business_connection_id, 'is_public>(&self, business_connection_id: &'business_connection_id str, is_public: Option<bool>) -> Result<ResponseContent<PostRemoveBusinessAccountProfilePhotoSuccess>, Error<PostRemoveBusinessAccountProfilePhotoError>>;

    /// POST /removeChatVerification
    ///
    /// Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn post_remove_chat_verification<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostRemoveChatVerificationSuccess>, Error<PostRemoveChatVerificationError>>;

    /// POST /removeUserVerification
    ///
    /// Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn post_remove_user_verification<'user_id>(&self, user_id: i32) -> Result<ResponseContent<PostRemoveUserVerificationSuccess>, Error<PostRemoveUserVerificationError>>;

    /// POST /reopenForumTopic
    ///
    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn post_reopen_forum_topic<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostReopenForumTopicSuccess>, Error<PostReopenForumTopicError>>;

    /// POST /reopenGeneralForumTopic
    ///
    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
    async fn post_reopen_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostReopenGeneralForumTopicSuccess>, Error<PostReopenGeneralForumTopicError>>;

    /// POST /replaceStickerInSet
    ///
    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
    async fn post_replace_sticker_in_set<'user_id, 'name, 'old_sticker, 'sticker>(&self, user_id: i32, name: &'name str, old_sticker: &'old_sticker str, sticker: models::models::InputSticker) -> Result<ResponseContent<PostReplaceStickerInSetSuccess>, Error<PostReplaceStickerInSetError>>;

    /// POST /restrictChatMember
    ///
    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
    async fn post_restrict_chat_member<'chat_id, 'user_id, 'permissions, 'use_independent_chat_permissions, 'until_date>(&self, chat_id: models::models::BotCommandScopeChatChatId, user_id: i32, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>, until_date: Option<i32>) -> Result<ResponseContent<PostRestrictChatMemberSuccess>, Error<PostRestrictChatMemberError>>;

    /// POST /revokeChatInviteLink
    ///
    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_revoke_chat_invite_link<'chat_id, 'invite_link>(&self, chat_id: models::models::RevokeChatInviteLinkRequestChatId, invite_link: &'invite_link str) -> Result<ResponseContent<PostRevokeChatInviteLinkSuccess>, Error<PostRevokeChatInviteLinkError>>;

    /// POST /savePreparedInlineMessage
    ///
    /// Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
    async fn post_save_prepared_inline_message<'user_id, 'result, 'allow_user_chats, 'allow_bot_chats, 'allow_group_chats, 'allow_channel_chats>(&self, user_id: i32, result: models::models::InlineQueryResult, allow_user_chats: Option<bool>, allow_bot_chats: Option<bool>, allow_group_chats: Option<bool>, allow_channel_chats: Option<bool>) -> Result<ResponseContent<PostSavePreparedInlineMessageSuccess>, Error<PostSavePreparedInlineMessageError>>;

    /// POST /sendAnimation
    ///
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_animation<'chat_id, 'animation, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, animation: Option<&'animation str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<&'thumbnail str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendAnimationSuccess>, Error<PostSendAnimationError>>;

    /// POST /sendAudio
    ///
    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.  For sending voice messages, use the [sendVoice](https://core.telegram.org/bots/api/#sendvoice) method instead.
    async fn post_send_audio<'chat_id, 'audio, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'performer, 'title, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, audio: Option<&'audio str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, performer: Option<&'performer str>, title: Option<&'title str>, thumbnail: Option<&'thumbnail str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendAudioSuccess>, Error<PostSendAudioError>>;

    /// POST /sendChatAction
    ///
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.  Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\\_photo*. The user will see a “sending photo” status for the bot.  We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    async fn post_send_chat_action<'chat_id, 'action, 'business_connection_id, 'message_thread_id>(&self, chat_id: models::models::SendMessageRequestChatId, action: &'action str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>) -> Result<ResponseContent<PostSendChatActionSuccess>, Error<PostSendChatActionError>>;

    /// POST /sendContact
    ///
    /// Use this method to send phone contacts. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_contact<'chat_id, 'phone_number, 'first_name, 'business_connection_id, 'message_thread_id, 'last_name, 'vcard, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, phone_number: &'phone_number str, first_name: &'first_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, last_name: Option<&'last_name str>, vcard: Option<&'vcard str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendContactSuccess>, Error<PostSendContactError>>;

    /// POST /sendDice
    ///
    /// Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_dice<'chat_id, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendDiceSuccess>, Error<PostSendDiceError>>;

    /// POST /sendDocument
    ///
    /// Use this method to send general files. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_document<'chat_id, 'document, 'business_connection_id, 'message_thread_id, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'disable_content_type_detection, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, document: Option<&'document str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, thumbnail: Option<&'thumbnail str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, disable_content_type_detection: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendDocumentSuccess>, Error<PostSendDocumentError>>;

    /// POST /sendGame
    ///
    /// Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_game<'chat_id, 'game_short_name, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: i32, game_short_name: &'game_short_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostSendGameSuccess>, Error<PostSendGameError>>;

    /// POST /sendGift
    ///
    /// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.
    async fn post_send_gift<'gift_id, 'user_id, 'chat_id, 'pay_for_upgrade, 'text, 'text_parse_mode, 'text_entities>(&self, gift_id: &'gift_id str, user_id: Option<i32>, chat_id: Option<models::models::SendGiftRequestChatId>, pay_for_upgrade: Option<bool>, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<PostSendGiftSuccess>, Error<PostSendGiftError>>;

    /// POST /sendInvoice
    ///
    /// Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_invoice<'chat_id, 'title, 'description, 'payload, 'currency, 'prices, 'message_thread_id, 'provider_token, 'max_tip_amount, 'suggested_tip_amounts, 'start_parameter, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, message_thread_id: Option<i32>, provider_token: Option<&'provider_token str>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, start_parameter: Option<&'start_parameter str>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostSendInvoiceSuccess>, Error<PostSendInvoiceError>>;

    /// POST /sendLocation
    ///
    /// Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_location<'chat_id, 'latitude, 'longitude, 'business_connection_id, 'message_thread_id, 'horizontal_accuracy, 'live_period, 'heading, 'proximity_alert_radius, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, horizontal_accuracy: Option<f64>, live_period: Option<i32>, heading: Option<i32>, proximity_alert_radius: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendLocationSuccess>, Error<PostSendLocationError>>;

    /// POST /sendMediaGroup
    ///
    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
    async fn post_send_media_group<'chat_id, 'media, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters>(&self, chat_id: models::models::SendMessageRequestChatId, media: Vec<models::SendMediaGroupRequestMediaInner>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>) -> Result<ResponseContent<PostSendMediaGroupSuccess>, Error<PostSendMediaGroupError>>;

    /// POST /sendMessage
    ///
    /// Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_message<'chat_id, 'text, 'business_connection_id, 'message_thread_id, 'parse_mode, 'entities, 'link_preview_options, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, text: &'text str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendMessageSuccess>, Error<PostSendMessageError>>;

    /// POST /sendPaidMedia
    ///
    /// Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_paid_media<'chat_id, 'star_count, 'media, 'business_connection_id, 'payload, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendPaidMediaRequestChatId, star_count: i32, media: Vec<models::InputPaidMedia>, business_connection_id: Option<&'business_connection_id str>, payload: Option<&'payload str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendPaidMediaSuccess>, Error<PostSendPaidMediaError>>;

    /// POST /sendPhoto
    ///
    /// Use this method to send photos. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_photo<'chat_id, 'photo, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, photo: Option<&'photo str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendPhotoSuccess>, Error<PostSendPhotoError>>;

    /// POST /sendPoll
    ///
    /// Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_poll<'chat_id, 'question, 'options, 'business_connection_id, 'message_thread_id, 'question_parse_mode, 'question_entities, 'is_anonymous, 'r_type, 'allows_multiple_answers, 'correct_option_id, 'explanation, 'explanation_parse_mode, 'explanation_entities, 'open_period, 'close_date, 'is_closed, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, question: &'question str, options: Vec<models::InputPollOption>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, question_parse_mode: Option<&'question_parse_mode str>, question_entities: Option<Vec<models::MessageEntity>>, is_anonymous: Option<bool>, r#type: Option<&'r_type str>, allows_multiple_answers: Option<bool>, correct_option_id: Option<i32>, explanation: Option<&'explanation str>, explanation_parse_mode: Option<&'explanation_parse_mode str>, explanation_entities: Option<Vec<models::MessageEntity>>, open_period: Option<i32>, close_date: Option<i32>, is_closed: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendPollSuccess>, Error<PostSendPollError>>;

    /// POST /sendSticker
    ///
    /// Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_sticker<'chat_id, 'sticker, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, sticker: Option<&'sticker str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendStickerSuccess>, Error<PostSendStickerError>>;

    /// POST /sendVenue
    ///
    /// Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_venue<'chat_id, 'latitude, 'longitude, 'title, 'address, 'business_connection_id, 'message_thread_id, 'foursquare_id, 'foursquare_type, 'google_place_id, 'google_place_type, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, latitude: f64, longitude: f64, title: &'title str, address: &'address str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, foursquare_id: Option<&'foursquare_id str>, foursquare_type: Option<&'foursquare_type str>, google_place_id: Option<&'google_place_id str>, google_place_type: Option<&'google_place_type str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVenueSuccess>, Error<PostSendVenueError>>;

    /// POST /sendVideo
    ///
    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_video<'chat_id, 'video, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'cover, 'start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'supports_streaming, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, video: Option<&'video str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<&'thumbnail str>, cover: Option<&'cover str>, start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, supports_streaming: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVideoSuccess>, Error<PostSendVideoError>>;

    /// POST /sendVideoNote
    ///
    /// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_video_note<'chat_id, 'video_note, 'business_connection_id, 'message_thread_id, 'duration, 'length, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, video_note: Option<&'video_note str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, length: Option<i32>, thumbnail: Option<&'thumbnail str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVideoNoteSuccess>, Error<PostSendVideoNoteError>>;

    /// POST /sendVoice
    ///
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as [Audio](https://core.telegram.org/bots/api/#audio) or [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_voice<'chat_id, 'voice, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, voice: Option<&'voice str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVoiceSuccess>, Error<PostSendVoiceError>>;

    /// POST /setBusinessAccountBio
    ///
    /// Changes the bio of a managed business account. Requires the *can\\_change\\_bio* business bot right. Returns *True* on success.
    async fn post_set_business_account_bio<'business_connection_id, 'bio>(&self, business_connection_id: &'business_connection_id str, bio: Option<&'bio str>) -> Result<ResponseContent<PostSetBusinessAccountBioSuccess>, Error<PostSetBusinessAccountBioError>>;

    /// POST /setBusinessAccountGiftSettings
    ///
    /// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the *can\\_change\\_gift\\_settings* business bot right. Returns *True* on success.
    async fn post_set_business_account_gift_settings<'business_connection_id, 'show_gift_button, 'accepted_gift_types>(&self, business_connection_id: &'business_connection_id str, show_gift_button: bool, accepted_gift_types: models::models::AcceptedGiftTypes) -> Result<ResponseContent<PostSetBusinessAccountGiftSettingsSuccess>, Error<PostSetBusinessAccountGiftSettingsError>>;

    /// POST /setBusinessAccountName
    ///
    /// Changes the first and last name of a managed business account. Requires the *can\\_change\\_name* business bot right. Returns *True* on success.
    async fn post_set_business_account_name<'business_connection_id, 'first_name, 'last_name>(&self, business_connection_id: &'business_connection_id str, first_name: &'first_name str, last_name: Option<&'last_name str>) -> Result<ResponseContent<PostSetBusinessAccountNameSuccess>, Error<PostSetBusinessAccountNameError>>;

    /// POST /setBusinessAccountProfilePhoto
    ///
    /// Changes the profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn post_set_business_account_profile_photo<'business_connection_id, 'photo, 'is_public>(&self, business_connection_id: &'business_connection_id str, photo: models::models::InputProfilePhoto, is_public: Option<bool>) -> Result<ResponseContent<PostSetBusinessAccountProfilePhotoSuccess>, Error<PostSetBusinessAccountProfilePhotoError>>;

    /// POST /setBusinessAccountUsername
    ///
    /// Changes the username of a managed business account. Requires the *can\\_change\\_username* business bot right. Returns *True* on success.
    async fn post_set_business_account_username<'business_connection_id, 'username>(&self, business_connection_id: &'business_connection_id str, username: Option<&'username str>) -> Result<ResponseContent<PostSetBusinessAccountUsernameSuccess>, Error<PostSetBusinessAccountUsernameError>>;

    /// POST /setChatAdministratorCustomTitle
    ///
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
    async fn post_set_chat_administrator_custom_title<'chat_id, 'user_id, 'custom_title>(&self, chat_id: models::models::BotCommandScopeChatChatId, user_id: i32, custom_title: &'custom_title str) -> Result<ResponseContent<PostSetChatAdministratorCustomTitleSuccess>, Error<PostSetChatAdministratorCustomTitleError>>;

    /// POST /setChatDescription
    ///
    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_set_chat_description<'chat_id, 'description>(&self, chat_id: models::models::SendMessageRequestChatId, description: Option<&'description str>) -> Result<ResponseContent<PostSetChatDescriptionSuccess>, Error<PostSetChatDescriptionError>>;

    /// POST /setChatMenuButton
    ///
    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
    async fn post_set_chat_menu_button<'chat_id, 'menu_button>(&self, chat_id: Option<i32>, menu_button: Option<models::models::MenuButton>) -> Result<ResponseContent<PostSetChatMenuButtonSuccess>, Error<PostSetChatMenuButtonError>>;

    /// POST /setChatPermissions
    ///
    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\\_restrict\\_members* administrator rights. Returns *True* on success.
    async fn post_set_chat_permissions<'chat_id, 'permissions, 'use_independent_chat_permissions>(&self, chat_id: models::models::BotCommandScopeChatChatId, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>) -> Result<ResponseContent<PostSetChatPermissionsSuccess>, Error<PostSetChatPermissionsError>>;

    /// POST /setChatPhoto
    ///
    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_set_chat_photo<'chat_id, 'photo>(&self, chat_id: models::models::SendMessageRequestChatId, photo: Option<models::serde_json::Value>) -> Result<ResponseContent<PostSetChatPhotoSuccess>, Error<PostSetChatPhotoError>>;

    /// POST /setChatStickerSet
    ///
    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn post_set_chat_sticker_set<'chat_id, 'sticker_set_name>(&self, chat_id: models::models::BotCommandScopeChatChatId, sticker_set_name: &'sticker_set_name str) -> Result<ResponseContent<PostSetChatStickerSetSuccess>, Error<PostSetChatStickerSetError>>;

    /// POST /setChatTitle
    ///
    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_set_chat_title<'chat_id, 'title>(&self, chat_id: models::models::SendMessageRequestChatId, title: &'title str) -> Result<ResponseContent<PostSetChatTitleSuccess>, Error<PostSetChatTitleError>>;

    /// POST /setCustomEmojiStickerSetThumbnail
    ///
    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
    async fn post_set_custom_emoji_sticker_set_thumbnail<'name, 'custom_emoji_id>(&self, name: &'name str, custom_emoji_id: Option<&'custom_emoji_id str>) -> Result<ResponseContent<PostSetCustomEmojiStickerSetThumbnailSuccess>, Error<PostSetCustomEmojiStickerSetThumbnailError>>;

    /// POST /setGameScore
    ///
    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
    async fn post_set_game_score<'user_id, 'score, 'force, 'disable_edit_message, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, score: i32, force: Option<bool>, disable_edit_message: Option<bool>, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<PostSetGameScoreSuccess>, Error<PostSetGameScoreError>>;

    /// POST /setMessageReaction
    ///
    /// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.
    async fn post_set_message_reaction<'chat_id, 'message_id, 'reaction, 'is_big>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32, reaction: Option<Vec<models::ReactionType>>, is_big: Option<bool>) -> Result<ResponseContent<PostSetMessageReactionSuccess>, Error<PostSetMessageReactionError>>;

    /// POST /setMyCommands
    ///
    /// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
    async fn post_set_my_commands<'commands, 'scope, 'language_code>(&self, commands: Vec<models::BotCommand>, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyCommandsSuccess>, Error<PostSetMyCommandsError>>;

    /// POST /setMyDefaultAdministratorRights
    ///
    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
    async fn post_set_my_default_administrator_rights<'rights, 'for_channels>(&self, rights: Option<models::models::ChatAdministratorRights>, for_channels: Option<bool>) -> Result<ResponseContent<PostSetMyDefaultAdministratorRightsSuccess>, Error<PostSetMyDefaultAdministratorRightsError>>;

    /// POST /setMyDescription
    ///
    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
    async fn post_set_my_description<'description, 'language_code>(&self, description: Option<&'description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyDescriptionSuccess>, Error<PostSetMyDescriptionError>>;

    /// POST /setMyName
    ///
    /// Use this method to change the bot's name. Returns *True* on success.
    async fn post_set_my_name<'name, 'language_code>(&self, name: Option<&'name str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyNameSuccess>, Error<PostSetMyNameError>>;

    /// POST /setMyShortDescription
    ///
    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
    async fn post_set_my_short_description<'short_description, 'language_code>(&self, short_description: Option<&'short_description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyShortDescriptionSuccess>, Error<PostSetMyShortDescriptionError>>;

    /// POST /setPassportDataErrors
    ///
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.  Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    async fn post_set_passport_data_errors<'user_id, 'errors>(&self, user_id: i32, errors: Vec<models::PassportElementError>) -> Result<ResponseContent<PostSetPassportDataErrorsSuccess>, Error<PostSetPassportDataErrorsError>>;

    /// POST /setStickerEmojiList
    ///
    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn post_set_sticker_emoji_list<'sticker, 'emoji_list>(&self, sticker: &'sticker str, emoji_list: Vec<String>) -> Result<ResponseContent<PostSetStickerEmojiListSuccess>, Error<PostSetStickerEmojiListError>>;

    /// POST /setStickerKeywords
    ///
    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn post_set_sticker_keywords<'sticker, 'keywords>(&self, sticker: &'sticker str, keywords: Option<Vec<String>>) -> Result<ResponseContent<PostSetStickerKeywordsSuccess>, Error<PostSetStickerKeywordsError>>;

    /// POST /setStickerMaskPosition
    ///
    /// Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
    async fn post_set_sticker_mask_position<'sticker, 'mask_position>(&self, sticker: &'sticker str, mask_position: Option<models::models::MaskPosition>) -> Result<ResponseContent<PostSetStickerMaskPositionSuccess>, Error<PostSetStickerMaskPositionError>>;

    /// POST /setStickerPositionInSet
    ///
    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
    async fn post_set_sticker_position_in_set<'sticker, 'position>(&self, sticker: &'sticker str, position: i32) -> Result<ResponseContent<PostSetStickerPositionInSetSuccess>, Error<PostSetStickerPositionInSetError>>;

    /// POST /setStickerSetThumbnail
    ///
    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
    async fn post_set_sticker_set_thumbnail<'name, 'user_id, 'format, 'thumbnail>(&self, name: &'name str, user_id: i32, format: &'format str, thumbnail: Option<&'thumbnail str>) -> Result<ResponseContent<PostSetStickerSetThumbnailSuccess>, Error<PostSetStickerSetThumbnailError>>;

    /// POST /setStickerSetTitle
    ///
    /// Use this method to set the title of a created sticker set. Returns *True* on success.
    async fn post_set_sticker_set_title<'name, 'title>(&self, name: &'name str, title: &'title str) -> Result<ResponseContent<PostSetStickerSetTitleSuccess>, Error<PostSetStickerSetTitleError>>;

    /// POST /setUserEmojiStatus
    ///
    /// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
    async fn post_set_user_emoji_status<'user_id, 'emoji_status_custom_emoji_id, 'emoji_status_expiration_date>(&self, user_id: i32, emoji_status_custom_emoji_id: Option<&'emoji_status_custom_emoji_id str>, emoji_status_expiration_date: Option<i32>) -> Result<ResponseContent<PostSetUserEmojiStatusSuccess>, Error<PostSetUserEmojiStatusError>>;

    /// POST /setWebhook
    ///
    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts. Returns *True* on success.  If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    async fn post_set_webhook<'url, 'certificate, 'ip_address, 'max_connections, 'allowed_updates, 'drop_pending_updates, 'secret_token>(&self, url: &'url str, certificate: Option<models::serde_json::Value>, ip_address: Option<&'ip_address str>, max_connections: Option<i32>, allowed_updates: Option<Vec<String>>, drop_pending_updates: Option<bool>, secret_token: Option<&'secret_token str>) -> Result<ResponseContent<PostSetWebhookSuccess>, Error<PostSetWebhookError>>;

    /// POST /stopMessageLiveLocation
    ///
    /// Use this method to stop updating a live location message before *live\\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn post_stop_message_live_location<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostStopMessageLiveLocationSuccess>, Error<PostStopMessageLiveLocationError>>;

    /// POST /stopPoll
    ///
    /// Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
    async fn post_stop_poll<'chat_id, 'message_id, 'business_connection_id, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostStopPollSuccess>, Error<PostStopPollError>>;

    /// POST /transferBusinessAccountStars
    ///
    /// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the *can\\_transfer\\_stars* business bot right. Returns *True* on success.
    async fn post_transfer_business_account_stars<'business_connection_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, star_count: i32) -> Result<ResponseContent<PostTransferBusinessAccountStarsSuccess>, Error<PostTransferBusinessAccountStarsError>>;

    /// POST /transferGift
    ///
    /// Transfers an owned unique gift to another user. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Requires *can\\_transfer\\_stars* business bot right if the transfer is paid. Returns *True* on success.
    async fn post_transfer_gift<'business_connection_id, 'owned_gift_id, 'new_owner_chat_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, new_owner_chat_id: i32, star_count: Option<i32>) -> Result<ResponseContent<PostTransferGiftSuccess>, Error<PostTransferGiftError>>;

    /// POST /unbanChatMember
    ///
    /// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\\_if\\_banned*. Returns *True* on success.
    async fn post_unban_chat_member<'chat_id, 'user_id, 'only_if_banned>(&self, chat_id: models::models::BanChatMemberRequestChatId, user_id: i32, only_if_banned: Option<bool>) -> Result<ResponseContent<PostUnbanChatMemberSuccess>, Error<PostUnbanChatMemberError>>;

    /// POST /unbanChatSenderChat
    ///
    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_unban_chat_sender_chat<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessageRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<PostUnbanChatSenderChatSuccess>, Error<PostUnbanChatSenderChatError>>;

    /// POST /unhideGeneralForumTopic
    ///
    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn post_unhide_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostUnhideGeneralForumTopicSuccess>, Error<PostUnhideGeneralForumTopicError>>;

    /// POST /unpinAllChatMessages
    ///
    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn post_unpin_all_chat_messages<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostUnpinAllChatMessagesSuccess>, Error<PostUnpinAllChatMessagesError>>;

    /// POST /unpinAllForumTopicMessages
    ///
    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn post_unpin_all_forum_topic_messages<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostUnpinAllForumTopicMessagesSuccess>, Error<PostUnpinAllForumTopicMessagesError>>;

    /// POST /unpinAllGeneralForumTopicMessages
    ///
    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn post_unpin_all_general_forum_topic_messages<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostUnpinAllGeneralForumTopicMessagesSuccess>, Error<PostUnpinAllGeneralForumTopicMessagesError>>;

    /// POST /unpinChatMessage
    ///
    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn post_unpin_chat_message<'chat_id, 'business_connection_id, 'message_id>(&self, chat_id: models::models::SendMessageRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_id: Option<i32>) -> Result<ResponseContent<PostUnpinChatMessageSuccess>, Error<PostUnpinChatMessageError>>;

    /// POST /upgradeGift
    ///
    /// Upgrades a given regular gift to a unique gift. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Additionally requires the *can\\_transfer\\_stars* business bot right if the upgrade is paid. Returns *True* on success.
    async fn post_upgrade_gift<'business_connection_id, 'owned_gift_id, 'keep_original_details, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, keep_original_details: Option<bool>, star_count: Option<i32>) -> Result<ResponseContent<PostUpgradeGiftSuccess>, Error<PostUpgradeGiftError>>;

    /// POST /uploadStickerFile
    ///
    /// Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
    async fn post_upload_sticker_file<'user_id, 'sticker, 'sticker_format>(&self, user_id: i32, sticker: Option<models::serde_json::Value>, sticker_format: &'sticker_format str) -> Result<ResponseContent<PostUploadStickerFileSuccess>, Error<PostUploadStickerFileError>>;

    /// POST /verifyChat
    ///
    /// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn post_verify_chat<'chat_id, 'custom_description>(&self, chat_id: models::models::SendMessageRequestChatId, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<PostVerifyChatSuccess>, Error<PostVerifyChatError>>;

    /// POST /verifyUser
    ///
    /// Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn post_verify_user<'user_id, 'custom_description>(&self, user_id: i32, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<PostVerifyUserSuccess>, Error<PostVerifyUserError>>;
}

pub struct DefaultApiClient {
    configuration: Arc<configuration::Configuration>
}

impl DefaultApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl DefaultApi for DefaultApiClient {
    /// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.
    async fn post_add_sticker_to_set<'user_id, 'name, 'sticker>(&self, user_id: i32, name: &'name str, sticker: models::models::InputSticker) -> Result<ResponseContent<PostAddStickerToSetSuccess>, Error<PostAddStickerToSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/addStickerToSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("user_id", user_id.to_string());
        local_var_form = local_var_form.text("name", name.to_string());
        local_var_form = local_var_form.text("sticker", sticker.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostAddStickerToSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostAddStickerToSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.  Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    async fn post_answer_callback_query<'callback_query_id, 'text, 'show_alert, 'url, 'cache_time>(&self, callback_query_id: &'callback_query_id str, text: Option<&'text str>, show_alert: Option<bool>, url: Option<&'url str>, cache_time: Option<i32>) -> Result<ResponseContent<PostAnswerCallbackQuerySuccess>, Error<PostAnswerCallbackQueryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/answerCallbackQuery", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("callback_query_id", callback_query_id.to_string());
        if let Some(local_var_param_value) = text {
            local_var_form_params.insert("text", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = show_alert {
            local_var_form_params.insert("show_alert", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = url {
            local_var_form_params.insert("url", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = cache_time {
            local_var_form_params.insert("cache_time", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostAnswerCallbackQuerySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostAnswerCallbackQueryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send answers to an inline query. On success, *True* is returned.   No more than **50** results per query are allowed.
    async fn post_answer_inline_query<'inline_query_id, 'results, 'cache_time, 'is_personal, 'next_offset, 'button>(&self, inline_query_id: &'inline_query_id str, results: Vec<models::InlineQueryResult>, cache_time: Option<i32>, is_personal: Option<bool>, next_offset: Option<&'next_offset str>, button: Option<models::models::InlineQueryResultsButton>) -> Result<ResponseContent<PostAnswerInlineQuerySuccess>, Error<PostAnswerInlineQueryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/answerInlineQuery", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("inline_query_id", inline_query_id.to_string());
        local_var_form_params.insert("results", results.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = cache_time {
            local_var_form_params.insert("cache_time", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = is_personal {
            local_var_form_params.insert("is_personal", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = next_offset {
            local_var_form_params.insert("next_offset", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = button {
            local_var_form_params.insert("button", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostAnswerInlineQuerySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostAnswerInlineQueryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\\_checkout\\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    async fn post_answer_pre_checkout_query<'pre_checkout_query_id, 'ok, 'error_message>(&self, pre_checkout_query_id: &'pre_checkout_query_id str, ok: bool, error_message: Option<&'error_message str>) -> Result<ResponseContent<PostAnswerPreCheckoutQuerySuccess>, Error<PostAnswerPreCheckoutQueryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/answerPreCheckoutQuery", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("pre_checkout_query_id", pre_checkout_query_id.to_string());
        local_var_form_params.insert("ok", ok.to_string());
        if let Some(local_var_param_value) = error_message {
            local_var_form_params.insert("error_message", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostAnswerPreCheckoutQuerySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostAnswerPreCheckoutQueryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// If you sent an invoice requesting a shipping address and the parameter *is\\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
    async fn post_answer_shipping_query<'shipping_query_id, 'ok, 'shipping_options, 'error_message>(&self, shipping_query_id: &'shipping_query_id str, ok: bool, shipping_options: Option<Vec<models::ShippingOption>>, error_message: Option<&'error_message str>) -> Result<ResponseContent<PostAnswerShippingQuerySuccess>, Error<PostAnswerShippingQueryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/answerShippingQuery", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("shipping_query_id", shipping_query_id.to_string());
        local_var_form_params.insert("ok", ok.to_string());
        if let Some(local_var_param_value) = shipping_options {
            local_var_form_params.insert("shipping_options", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = error_message {
            local_var_form_params.insert("error_message", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostAnswerShippingQuerySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostAnswerShippingQueryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
    async fn post_answer_web_app_query<'web_app_query_id, 'result>(&self, web_app_query_id: &'web_app_query_id str, result: models::models::InlineQueryResult) -> Result<ResponseContent<PostAnswerWebAppQuerySuccess>, Error<PostAnswerWebAppQueryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/answerWebAppQuery", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("web_app_query_id", web_app_query_id.to_string());
        local_var_form_params.insert("result", result.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostAnswerWebAppQuerySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostAnswerWebAppQueryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn post_approve_chat_join_request<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessageRequestChatId, user_id: i32) -> Result<ResponseContent<PostApproveChatJoinRequestSuccess>, Error<PostApproveChatJoinRequestError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/approveChatJoinRequest", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostApproveChatJoinRequestSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostApproveChatJoinRequestError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_ban_chat_member<'chat_id, 'user_id, 'until_date, 'revoke_messages>(&self, chat_id: models::models::BanChatMemberRequestChatId, user_id: i32, until_date: Option<i32>, revoke_messages: Option<bool>) -> Result<ResponseContent<PostBanChatMemberSuccess>, Error<PostBanChatMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/banChatMember", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = until_date {
            local_var_form_params.insert("until_date", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = revoke_messages {
            local_var_form_params.insert("revoke_messages", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostBanChatMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostBanChatMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_ban_chat_sender_chat<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessageRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<PostBanChatSenderChatSuccess>, Error<PostBanChatSenderChatError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/banChatSenderChat", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("sender_chat_id", sender_chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostBanChatSenderChatSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostBanChatSenderChatError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
    async fn post_close<>(&self, ) -> Result<ResponseContent<PostCloseSuccess>, Error<PostCloseError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/close", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCloseSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCloseError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn post_close_forum_topic<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostCloseForumTopicSuccess>, Error<PostCloseForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/closeForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_thread_id", message_thread_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCloseForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCloseForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn post_close_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostCloseGeneralForumTopicSuccess>, Error<PostCloseGeneralForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/closeGeneralForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCloseGeneralForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCloseGeneralForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Converts a given regular gift to Telegram Stars. Requires the *can\\_convert\\_gifts\\_to\\_stars* business bot right. Returns *True* on success.
    async fn post_convert_gift_to_stars<'business_connection_id, 'owned_gift_id>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str) -> Result<ResponseContent<PostConvertGiftToStarsSuccess>, Error<PostConvertGiftToStarsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/convertGiftToStars", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("owned_gift_id", owned_gift_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostConvertGiftToStarsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostConvertGiftToStarsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api/#forwardmessage), but the copied message doesn't have a link to the original message. Returns the [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent message on success.
    async fn post_copy_message<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessageRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostCopyMessageSuccess>, Error<PostCopyMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/copyMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("from_chat_id", from_chat_id.to_string());
        local_var_form_params.insert("message_id", message_id.to_string());
        if let Some(local_var_param_value) = video_start_timestamp {
            local_var_form_params.insert("video_start_timestamp", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption {
            local_var_form_params.insert("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form_params.insert("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form_params.insert("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = show_caption_above_media {
            local_var_form_params.insert("show_caption_above_media", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCopyMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCopyMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn post_copy_messages<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content, 'remove_caption>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessagesRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, remove_caption: Option<bool>) -> Result<ResponseContent<PostCopyMessagesSuccess>, Error<PostCopyMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/copyMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("from_chat_id", from_chat_id.to_string());
        local_var_form_params.insert("message_ids", message_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = remove_caption {
            local_var_form_params.insert("remove_caption", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCopyMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCopyMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_create_chat_invite_link<'chat_id, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessageRequestChatId, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<PostCreateChatInviteLinkSuccess>, Error<PostCreateChatInviteLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/createChatInviteLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = name {
            local_var_form_params.insert("name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = expire_date {
            local_var_form_params.insert("expire_date", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = member_limit {
            local_var_form_params.insert("member_limit", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = creates_join_request {
            local_var_form_params.insert("creates_join_request", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCreateChatInviteLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCreateChatInviteLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\\_invite\\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_create_chat_subscription_invite_link<'chat_id, 'subscription_period, 'subscription_price, 'name>(&self, chat_id: models::models::CreateChatSubscriptionInviteLinkRequestChatId, subscription_period: i32, subscription_price: i32, name: Option<&'name str>) -> Result<ResponseContent<PostCreateChatSubscriptionInviteLinkSuccess>, Error<PostCreateChatSubscriptionInviteLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/createChatSubscriptionInviteLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = name {
            local_var_form_params.insert("name", local_var_param_value.to_string());
        }
        local_var_form_params.insert("subscription_period", subscription_period.to_string());
        local_var_form_params.insert("subscription_price", subscription_price.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCreateChatSubscriptionInviteLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCreateChatSubscriptionInviteLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
    async fn post_create_forum_topic<'chat_id, 'name, 'icon_color, 'icon_custom_emoji_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, name: &'name str, icon_color: Option<i32>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<PostCreateForumTopicSuccess>, Error<PostCreateForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/createForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("name", name.to_string());
        if let Some(local_var_param_value) = icon_color {
            local_var_form_params.insert("icon_color", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = icon_custom_emoji_id {
            local_var_form_params.insert("icon_custom_emoji_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCreateForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCreateForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
    async fn post_create_invoice_link<'title, 'description, 'payload, 'currency, 'prices, 'business_connection_id, 'provider_token, 'subscription_period, 'max_tip_amount, 'suggested_tip_amounts, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible>(&self, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, business_connection_id: Option<&'business_connection_id str>, provider_token: Option<&'provider_token str>, subscription_period: Option<i32>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>) -> Result<ResponseContent<PostCreateInvoiceLinkSuccess>, Error<PostCreateInvoiceLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/createInvoiceLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("title", title.to_string());
        local_var_form_params.insert("description", description.to_string());
        local_var_form_params.insert("payload", payload.to_string());
        if let Some(local_var_param_value) = provider_token {
            local_var_form_params.insert("provider_token", local_var_param_value.to_string());
        }
        local_var_form_params.insert("currency", currency.to_string());
        local_var_form_params.insert("prices", prices.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = subscription_period {
            local_var_form_params.insert("subscription_period", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = max_tip_amount {
            local_var_form_params.insert("max_tip_amount", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = suggested_tip_amounts {
            local_var_form_params.insert("suggested_tip_amounts", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = provider_data {
            local_var_form_params.insert("provider_data", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_url {
            local_var_form_params.insert("photo_url", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_size {
            local_var_form_params.insert("photo_size", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_width {
            local_var_form_params.insert("photo_width", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_height {
            local_var_form_params.insert("photo_height", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_name {
            local_var_form_params.insert("need_name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_phone_number {
            local_var_form_params.insert("need_phone_number", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_email {
            local_var_form_params.insert("need_email", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_shipping_address {
            local_var_form_params.insert("need_shipping_address", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = send_phone_number_to_provider {
            local_var_form_params.insert("send_phone_number_to_provider", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = send_email_to_provider {
            local_var_form_params.insert("send_email_to_provider", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = is_flexible {
            local_var_form_params.insert("is_flexible", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCreateInvoiceLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCreateInvoiceLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
    async fn post_create_new_sticker_set<'user_id, 'name, 'title, 'stickers, 'sticker_type, 'needs_repainting>(&self, user_id: i32, name: &'name str, title: &'title str, stickers: Vec<models::InputSticker>, sticker_type: Option<&'sticker_type str>, needs_repainting: Option<bool>) -> Result<ResponseContent<PostCreateNewStickerSetSuccess>, Error<PostCreateNewStickerSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/createNewStickerSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("user_id", user_id.to_string());
        local_var_form = local_var_form.text("name", name.to_string());
        local_var_form = local_var_form.text("title", title.to_string());
        local_var_form = local_var_form.text("stickers", stickers.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = sticker_type {
            local_var_form = local_var_form.text("sticker_type", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = needs_repainting {
            local_var_form = local_var_form.text("needs_repainting", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostCreateNewStickerSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostCreateNewStickerSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn post_decline_chat_join_request<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessageRequestChatId, user_id: i32) -> Result<ResponseContent<PostDeclineChatJoinRequestSuccess>, Error<PostDeclineChatJoinRequestError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/declineChatJoinRequest", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeclineChatJoinRequestSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeclineChatJoinRequestError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete messages on behalf of a business account. Requires the *can\\_delete\\_outgoing\\_messages* business bot right to delete messages sent by the bot itself, or the *can\\_delete\\_all\\_messages* business bot right to delete any message. Returns *True* on success.
    async fn post_delete_business_messages<'business_connection_id, 'message_ids>(&self, business_connection_id: &'business_connection_id str, message_ids: Vec<i32>) -> Result<ResponseContent<PostDeleteBusinessMessagesSuccess>, Error<PostDeleteBusinessMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteBusinessMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("message_ids", message_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteBusinessMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteBusinessMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_delete_chat_photo<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostDeleteChatPhotoSuccess>, Error<PostDeleteChatPhotoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteChatPhoto", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteChatPhotoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteChatPhotoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn post_delete_chat_sticker_set<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostDeleteChatStickerSetSuccess>, Error<PostDeleteChatStickerSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteChatStickerSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteChatStickerSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteChatStickerSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_delete\\_messages* administrator rights. Returns *True* on success.
    async fn post_delete_forum_topic<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostDeleteForumTopicSuccess>, Error<PostDeleteForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_thread_id", message_thread_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a message, including service messages, with the following limitations:   \\- A message can only be deleted if it was sent less than 48 hours ago.   \\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.   \\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.   \\- Bots can delete outgoing messages in private chats, groups, and supergroups.   \\- Bots can delete incoming messages in private chats.   \\- Bots granted *can\\_post\\_messages* permissions can delete outgoing messages in channels.   \\- If the bot is an administrator of a group, it can delete any message there.   \\- If the bot has *can\\_delete\\_messages* permission in a supergroup or a channel, it can delete any message there.   Returns *True* on success.
    async fn post_delete_message<'chat_id, 'message_id>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32) -> Result<ResponseContent<PostDeleteMessageSuccess>, Error<PostDeleteMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_id", message_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
    async fn post_delete_messages<'chat_id, 'message_ids>(&self, chat_id: models::models::SendMessageRequestChatId, message_ids: Vec<i32>) -> Result<ResponseContent<PostDeleteMessagesSuccess>, Error<PostDeleteMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_ids", message_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
    async fn post_delete_my_commands<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostDeleteMyCommandsSuccess>, Error<PostDeleteMyCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteMyCommands", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = scope {
            local_var_form_params.insert("scope", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteMyCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteMyCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
    async fn post_delete_sticker_from_set<'sticker>(&self, sticker: &'sticker str) -> Result<ResponseContent<PostDeleteStickerFromSetSuccess>, Error<PostDeleteStickerFromSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteStickerFromSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("sticker", sticker.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteStickerFromSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteStickerFromSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
    async fn post_delete_sticker_set<'name>(&self, name: &'name str) -> Result<ResponseContent<PostDeleteStickerSetSuccess>, Error<PostDeleteStickerSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteStickerSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("name", name.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteStickerSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteStickerSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns *True* on success.
    async fn post_delete_story<'business_connection_id, 'story_id>(&self, business_connection_id: &'business_connection_id str, story_id: i32) -> Result<ResponseContent<PostDeleteStorySuccess>, Error<PostDeleteStoryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteStory", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("story_id", story_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteStorySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteStoryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
    async fn post_delete_webhook<'drop_pending_updates>(&self, drop_pending_updates: Option<bool>) -> Result<ResponseContent<PostDeleteWebhookSuccess>, Error<PostDeleteWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/deleteWebhook", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = drop_pending_updates {
            local_var_form_params.insert("drop_pending_updates", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostDeleteWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostDeleteWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_edit_chat_invite_link<'chat_id, 'invite_link, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessageRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<PostEditChatInviteLinkSuccess>, Error<PostEditChatInviteLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editChatInviteLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("invite_link", invite_link.to_string());
        if let Some(local_var_param_value) = name {
            local_var_form_params.insert("name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = expire_date {
            local_var_form_params.insert("expire_date", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = member_limit {
            local_var_form_params.insert("member_limit", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = creates_join_request {
            local_var_form_params.insert("creates_join_request", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditChatInviteLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditChatInviteLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit a subscription invite link created by the bot. The bot must have the *can\\_invite\\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_edit_chat_subscription_invite_link<'chat_id, 'invite_link, 'name>(&self, chat_id: models::models::SendMessageRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>) -> Result<ResponseContent<PostEditChatSubscriptionInviteLinkSuccess>, Error<PostEditChatSubscriptionInviteLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editChatSubscriptionInviteLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("invite_link", invite_link.to_string());
        if let Some(local_var_param_value) = name {
            local_var_form_params.insert("name", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditChatSubscriptionInviteLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditChatSubscriptionInviteLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn post_edit_forum_topic<'chat_id, 'message_thread_id, 'name, 'icon_custom_emoji_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32, name: Option<&'name str>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<PostEditForumTopicSuccess>, Error<PostEditForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_thread_id", message_thread_id.to_string());
        if let Some(local_var_param_value) = name {
            local_var_form_params.insert("name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = icon_custom_emoji_id {
            local_var_form_params.insert("icon_custom_emoji_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn post_edit_general_forum_topic<'chat_id, 'name>(&self, chat_id: models::models::BotCommandScopeChatChatId, name: &'name str) -> Result<ResponseContent<PostEditGeneralForumTopicSuccess>, Error<PostEditGeneralForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editGeneralForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("name", name.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditGeneralForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditGeneralForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_caption<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageCaptionSuccess>, Error<PostEditMessageCaptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editMessageCaption", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption {
            local_var_form_params.insert("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form_params.insert("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form_params.insert("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = show_caption_above_media {
            local_var_form_params.insert("show_caption_above_media", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditMessageCaptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditMessageCaptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit live location messages. A location can be edited until its *live\\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn post_edit_message_live_location<'latitude, 'longitude, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'live_period, 'horizontal_accuracy, 'heading, 'proximity_alert_radius, 'reply_markup>(&self, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, live_period: Option<i32>, horizontal_accuracy: Option<f64>, heading: Option<i32>, proximity_alert_radius: Option<i32>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageLiveLocationSuccess>, Error<PostEditMessageLiveLocationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editMessageLiveLocation", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("latitude", latitude.to_string());
        local_var_form_params.insert("longitude", longitude.to_string());
        if let Some(local_var_param_value) = live_period {
            local_var_form_params.insert("live_period", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = horizontal_accuracy {
            local_var_form_params.insert("horizontal_accuracy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = heading {
            local_var_form_params.insert("heading", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = proximity_alert_radius {
            local_var_form_params.insert("proximity_alert_radius", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditMessageLiveLocationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditMessageLiveLocationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_media<'media, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, media: models::models::InputMedia, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageMediaSuccess>, Error<PostEditMessageMediaError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editMessageMedia", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form = local_var_form.text("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form = local_var_form.text("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form = local_var_form.text("inline_message_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("media", media.to_string());
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditMessageMediaSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditMessageMediaError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_reply_markup<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageReplyMarkupSuccess>, Error<PostEditMessageReplyMarkupError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editMessageReplyMarkup", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditMessageReplyMarkupSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditMessageReplyMarkupError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn post_edit_message_text<'text, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'parse_mode, 'entities, 'link_preview_options, 'reply_markup>(&self, text: &'text str, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostEditMessageTextSuccess>, Error<PostEditMessageTextError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editMessageText", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("text", text.to_string());
        if let Some(local_var_param_value) = parse_mode {
            local_var_form_params.insert("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = entities {
            local_var_form_params.insert("entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = link_preview_options {
            local_var_form_params.insert("link_preview_options", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditMessageTextSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditMessageTextError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Edits a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn post_edit_story<'business_connection_id, 'story_id, 'content, 'caption, 'parse_mode, 'caption_entities, 'areas>(&self, business_connection_id: &'business_connection_id str, story_id: i32, content: models::models::InputStoryContent, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>) -> Result<ResponseContent<PostEditStorySuccess>, Error<PostEditStoryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editStory", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("business_connection_id", business_connection_id.to_string());
        local_var_form = local_var_form.text("story_id", story_id.to_string());
        local_var_form = local_var_form.text("content", content.to_string());
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = areas {
            local_var_form = local_var_form.text("areas", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditStorySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditStoryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
    async fn post_edit_user_star_subscription<'user_id, 'telegram_payment_charge_id, 'is_canceled>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str, is_canceled: bool) -> Result<ResponseContent<PostEditUserStarSubscriptionSuccess>, Error<PostEditUserStarSubscriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/editUserStarSubscription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("telegram_payment_charge_id", telegram_payment_charge_id.to_string());
        local_var_form_params.insert("is_canceled", is_canceled.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostEditUserStarSubscriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostEditUserStarSubscriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
    async fn post_export_chat_invite_link<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostExportChatInviteLinkSuccess>, Error<PostExportChatInviteLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/exportChatInviteLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostExportChatInviteLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostExportChatInviteLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_forward_message<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessageRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostForwardMessageSuccess>, Error<PostForwardMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/forwardMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("from_chat_id", from_chat_id.to_string());
        if let Some(local_var_param_value) = video_start_timestamp {
            local_var_form_params.insert("video_start_timestamp", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        local_var_form_params.insert("message_id", message_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostForwardMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostForwardMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn post_forward_messages<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessageRequestChatId, from_chat_id: models::models::ForwardMessagesRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostForwardMessagesSuccess>, Error<PostForwardMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/forwardMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("from_chat_id", from_chat_id.to_string());
        local_var_form_params.insert("message_ids", message_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostForwardMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostForwardMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
    async fn post_get_available_gifts<>(&self, ) -> Result<ResponseContent<PostGetAvailableGiftsSuccess>, Error<PostGetAvailableGiftsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getAvailableGifts", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetAvailableGiftsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetAvailableGiftsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the gifts received and owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [OwnedGifts](https://core.telegram.org/bots/api/#ownedgifts) on success.
    async fn post_get_business_account_gifts<'business_connection_id, 'exclude_unsaved, 'exclude_saved, 'exclude_unlimited, 'exclude_limited, 'exclude_unique, 'sort_by_price, 'offset, 'limit>(&self, business_connection_id: &'business_connection_id str, exclude_unsaved: Option<bool>, exclude_saved: Option<bool>, exclude_unlimited: Option<bool>, exclude_limited: Option<bool>, exclude_unique: Option<bool>, sort_by_price: Option<bool>, offset: Option<&'offset str>, limit: Option<i32>) -> Result<ResponseContent<PostGetBusinessAccountGiftsSuccess>, Error<PostGetBusinessAccountGiftsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getBusinessAccountGifts", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        if let Some(local_var_param_value) = exclude_unsaved {
            local_var_form_params.insert("exclude_unsaved", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = exclude_saved {
            local_var_form_params.insert("exclude_saved", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = exclude_unlimited {
            local_var_form_params.insert("exclude_unlimited", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = exclude_limited {
            local_var_form_params.insert("exclude_limited", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = exclude_unique {
            local_var_form_params.insert("exclude_unique", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = sort_by_price {
            local_var_form_params.insert("sort_by_price", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = offset {
            local_var_form_params.insert("offset", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = limit {
            local_var_form_params.insert("limit", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetBusinessAccountGiftsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetBusinessAccountGiftsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [StarAmount](https://core.telegram.org/bots/api/#staramount) on success.
    async fn post_get_business_account_star_balance<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<PostGetBusinessAccountStarBalanceSuccess>, Error<PostGetBusinessAccountStarBalanceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getBusinessAccountStarBalance", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetBusinessAccountStarBalanceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetBusinessAccountStarBalanceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
    async fn post_get_business_connection<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<PostGetBusinessConnectionSuccess>, Error<PostGetBusinessConnectionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getBusinessConnection", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetBusinessConnectionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetBusinessConnectionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
    async fn post_get_chat<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostGetChatSuccess>, Error<PostGetChatError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getChat", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetChatSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetChatError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
    async fn post_get_chat_administrators<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostGetChatAdministratorsSuccess>, Error<PostGetChatAdministratorsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getChatAdministrators", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetChatAdministratorsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetChatAdministratorsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
    async fn post_get_chat_member<'chat_id, 'user_id>(&self, chat_id: models::models::LeaveChatRequestChatId, user_id: i32) -> Result<ResponseContent<PostGetChatMemberSuccess>, Error<PostGetChatMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getChatMember", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetChatMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetChatMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the number of members in a chat. Returns *Int* on success.
    async fn post_get_chat_member_count<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostGetChatMemberCountSuccess>, Error<PostGetChatMemberCountError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getChatMemberCount", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetChatMemberCountSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetChatMemberCountError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
    async fn post_get_chat_menu_button<'chat_id>(&self, chat_id: Option<i32>) -> Result<ResponseContent<PostGetChatMenuButtonSuccess>, Error<PostGetChatMenuButtonError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getChatMenuButton", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetChatMenuButtonSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetChatMenuButtonError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn post_get_custom_emoji_stickers<'custom_emoji_ids>(&self, custom_emoji_ids: Vec<String>) -> Result<ResponseContent<PostGetCustomEmojiStickersSuccess>, Error<PostGetCustomEmojiStickersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getCustomEmojiStickers", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("custom_emoji_ids", custom_emoji_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetCustomEmojiStickersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetCustomEmojiStickersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
    async fn post_get_file<'file_id>(&self, file_id: &'file_id str) -> Result<ResponseContent<PostGetFileSuccess>, Error<PostGetFileError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getFile", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("file_id", file_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetFileSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetFileError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn post_get_forum_topic_icon_stickers<>(&self, ) -> Result<ResponseContent<PostGetForumTopicIconStickersSuccess>, Error<PostGetForumTopicIconStickersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getForumTopicIconStickers", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetForumTopicIconStickersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetForumTopicIconStickersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.  This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
    async fn post_get_game_high_scores<'user_id, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<PostGetGameHighScoresSuccess>, Error<PostGetGameHighScoresError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getGameHighScores", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetGameHighScoresSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetGameHighScoresError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
    async fn post_get_me<>(&self, ) -> Result<ResponseContent<PostGetMeSuccess>, Error<PostGetMeError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getMe", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetMeSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetMeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
    async fn post_get_my_commands<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyCommandsSuccess>, Error<PostGetMyCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getMyCommands", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = scope {
            local_var_form_params.insert("scope", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetMyCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetMyCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
    async fn post_get_my_default_administrator_rights<'for_channels>(&self, for_channels: Option<bool>) -> Result<ResponseContent<PostGetMyDefaultAdministratorRightsSuccess>, Error<PostGetMyDefaultAdministratorRightsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getMyDefaultAdministratorRights", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = for_channels {
            local_var_form_params.insert("for_channels", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetMyDefaultAdministratorRightsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetMyDefaultAdministratorRightsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
    async fn post_get_my_description<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyDescriptionSuccess>, Error<PostGetMyDescriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getMyDescription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetMyDescriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetMyDescriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
    async fn post_get_my_name<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyNameSuccess>, Error<PostGetMyNameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getMyName", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetMyNameSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetMyNameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
    async fn post_get_my_short_description<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostGetMyShortDescriptionSuccess>, Error<PostGetMyShortDescriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getMyShortDescription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetMyShortDescriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetMyShortDescriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
    async fn post_get_star_transactions<'offset, 'limit>(&self, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<PostGetStarTransactionsSuccess>, Error<PostGetStarTransactionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getStarTransactions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = offset {
            local_var_form_params.insert("offset", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = limit {
            local_var_form_params.insert("limit", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetStarTransactionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetStarTransactionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
    async fn post_get_sticker_set<'name>(&self, name: &'name str) -> Result<ResponseContent<PostGetStickerSetSuccess>, Error<PostGetStickerSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getStickerSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("name", name.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetStickerSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetStickerSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.
    async fn post_get_updates<'offset, 'limit, 'timeout, 'allowed_updates>(&self, offset: Option<i32>, limit: Option<i32>, timeout: Option<i32>, allowed_updates: Option<Vec<String>>) -> Result<ResponseContent<PostGetUpdatesSuccess>, Error<PostGetUpdatesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getUpdates", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = offset {
            local_var_form_params.insert("offset", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = limit {
            local_var_form_params.insert("limit", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = timeout {
            local_var_form_params.insert("timeout", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allowed_updates {
            local_var_form_params.insert("allowed_updates", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetUpdatesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetUpdatesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
    async fn post_get_user_chat_boosts<'chat_id, 'user_id>(&self, chat_id: models::models::GetUserChatBoostsRequestChatId, user_id: i32) -> Result<ResponseContent<PostGetUserChatBoostsSuccess>, Error<PostGetUserChatBoostsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getUserChatBoosts", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetUserChatBoostsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetUserChatBoostsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
    async fn post_get_user_profile_photos<'user_id, 'offset, 'limit>(&self, user_id: i32, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<PostGetUserProfilePhotosSuccess>, Error<PostGetUserProfilePhotosError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getUserProfilePhotos", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = offset {
            local_var_form_params.insert("offset", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = limit {
            local_var_form_params.insert("limit", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetUserProfilePhotosSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetUserProfilePhotosError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
    async fn post_get_webhook_info<>(&self, ) -> Result<ResponseContent<PostGetWebhookInfoSuccess>, Error<PostGetWebhookInfoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/getWebhookInfo", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGetWebhookInfoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGetWebhookInfoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Gifts a Telegram Premium subscription to the given user. Returns *True* on success.
    async fn post_gift_premium_subscription<'user_id, 'month_count, 'star_count, 'text, 'text_parse_mode, 'text_entities>(&self, user_id: i32, month_count: i32, star_count: i32, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<PostGiftPremiumSubscriptionSuccess>, Error<PostGiftPremiumSubscriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/giftPremiumSubscription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("month_count", month_count.to_string());
        local_var_form_params.insert("star_count", star_count.to_string());
        if let Some(local_var_param_value) = text {
            local_var_form_params.insert("text", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = text_parse_mode {
            local_var_form_params.insert("text_parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = text_entities {
            local_var_form_params.insert("text_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostGiftPremiumSubscriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostGiftPremiumSubscriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
    async fn post_hide_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostHideGeneralForumTopicSuccess>, Error<PostHideGeneralForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/hideGeneralForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostHideGeneralForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostHideGeneralForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    async fn post_leave_chat<'chat_id>(&self, chat_id: models::models::LeaveChatRequestChatId) -> Result<ResponseContent<PostLeaveChatSuccess>, Error<PostLeaveChatError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/leaveChat", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostLeaveChatSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostLeaveChatError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
    async fn post_log_out<>(&self, ) -> Result<ResponseContent<PostLogOutSuccess>, Error<PostLogOutError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/logOut", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostLogOutSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostLogOutError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn post_pin_chat_message<'chat_id, 'message_id, 'business_connection_id, 'disable_notification>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, disable_notification: Option<bool>) -> Result<ResponseContent<PostPinChatMessageSuccess>, Error<PostPinChatMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/pinChatMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_id", message_id.to_string());
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostPinChatMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostPinChatMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Posts a story on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn post_post_story<'business_connection_id, 'content, 'active_period, 'caption, 'parse_mode, 'caption_entities, 'areas, 'post_to_chat_page, 'protect_content>(&self, business_connection_id: &'business_connection_id str, content: models::models::InputStoryContent, active_period: i32, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>, post_to_chat_page: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostPostStorySuccess>, Error<PostPostStoryError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/postStory", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("business_connection_id", business_connection_id.to_string());
        local_var_form = local_var_form.text("content", content.to_string());
        local_var_form = local_var_form.text("active_period", active_period.to_string());
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = areas {
            local_var_form = local_var_form.text("areas", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = post_to_chat_page {
            local_var_form = local_var_form.text("post_to_chat_page", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostPostStorySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostPostStoryError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    async fn post_promote_chat_member<'chat_id, 'user_id, 'is_anonymous, 'can_manage_chat, 'can_delete_messages, 'can_manage_video_chats, 'can_restrict_members, 'can_promote_members, 'can_change_info, 'can_invite_users, 'can_post_stories, 'can_edit_stories, 'can_delete_stories, 'can_post_messages, 'can_edit_messages, 'can_pin_messages, 'can_manage_topics>(&self, chat_id: models::models::SendMessageRequestChatId, user_id: i32, is_anonymous: Option<bool>, can_manage_chat: Option<bool>, can_delete_messages: Option<bool>, can_manage_video_chats: Option<bool>, can_restrict_members: Option<bool>, can_promote_members: Option<bool>, can_change_info: Option<bool>, can_invite_users: Option<bool>, can_post_stories: Option<bool>, can_edit_stories: Option<bool>, can_delete_stories: Option<bool>, can_post_messages: Option<bool>, can_edit_messages: Option<bool>, can_pin_messages: Option<bool>, can_manage_topics: Option<bool>) -> Result<ResponseContent<PostPromoteChatMemberSuccess>, Error<PostPromoteChatMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/promoteChatMember", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = is_anonymous {
            local_var_form_params.insert("is_anonymous", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_manage_chat {
            local_var_form_params.insert("can_manage_chat", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_delete_messages {
            local_var_form_params.insert("can_delete_messages", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_manage_video_chats {
            local_var_form_params.insert("can_manage_video_chats", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_restrict_members {
            local_var_form_params.insert("can_restrict_members", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_promote_members {
            local_var_form_params.insert("can_promote_members", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_change_info {
            local_var_form_params.insert("can_change_info", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_invite_users {
            local_var_form_params.insert("can_invite_users", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_post_stories {
            local_var_form_params.insert("can_post_stories", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_edit_stories {
            local_var_form_params.insert("can_edit_stories", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_delete_stories {
            local_var_form_params.insert("can_delete_stories", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_post_messages {
            local_var_form_params.insert("can_post_messages", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_edit_messages {
            local_var_form_params.insert("can_edit_messages", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_pin_messages {
            local_var_form_params.insert("can_pin_messages", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = can_manage_topics {
            local_var_form_params.insert("can_manage_topics", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostPromoteChatMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostPromoteChatMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Marks incoming message as read on behalf of a business account. Requires the *can\\_read\\_messages* business bot right. Returns *True* on success.
    async fn post_read_business_message<'business_connection_id, 'chat_id, 'message_id>(&self, business_connection_id: &'business_connection_id str, chat_id: i32, message_id: i32) -> Result<ResponseContent<PostReadBusinessMessageSuccess>, Error<PostReadBusinessMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/readBusinessMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_id", message_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostReadBusinessMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostReadBusinessMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
    async fn post_refund_star_payment<'user_id, 'telegram_payment_charge_id>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str) -> Result<ResponseContent<PostRefundStarPaymentSuccess>, Error<PostRefundStarPaymentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/refundStarPayment", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("telegram_payment_charge_id", telegram_payment_charge_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostRefundStarPaymentSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostRefundStarPaymentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Removes the current profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn post_remove_business_account_profile_photo<'business_connection_id, 'is_public>(&self, business_connection_id: &'business_connection_id str, is_public: Option<bool>) -> Result<ResponseContent<PostRemoveBusinessAccountProfilePhotoSuccess>, Error<PostRemoveBusinessAccountProfilePhotoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/removeBusinessAccountProfilePhoto", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        if let Some(local_var_param_value) = is_public {
            local_var_form_params.insert("is_public", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostRemoveBusinessAccountProfilePhotoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostRemoveBusinessAccountProfilePhotoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn post_remove_chat_verification<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostRemoveChatVerificationSuccess>, Error<PostRemoveChatVerificationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/removeChatVerification", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostRemoveChatVerificationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostRemoveChatVerificationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn post_remove_user_verification<'user_id>(&self, user_id: i32) -> Result<ResponseContent<PostRemoveUserVerificationSuccess>, Error<PostRemoveUserVerificationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/removeUserVerification", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostRemoveUserVerificationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostRemoveUserVerificationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn post_reopen_forum_topic<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostReopenForumTopicSuccess>, Error<PostReopenForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/reopenForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_thread_id", message_thread_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostReopenForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostReopenForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
    async fn post_reopen_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostReopenGeneralForumTopicSuccess>, Error<PostReopenGeneralForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/reopenGeneralForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostReopenGeneralForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostReopenGeneralForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
    async fn post_replace_sticker_in_set<'user_id, 'name, 'old_sticker, 'sticker>(&self, user_id: i32, name: &'name str, old_sticker: &'old_sticker str, sticker: models::models::InputSticker) -> Result<ResponseContent<PostReplaceStickerInSetSuccess>, Error<PostReplaceStickerInSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/replaceStickerInSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("user_id", user_id.to_string());
        local_var_form = local_var_form.text("name", name.to_string());
        local_var_form = local_var_form.text("old_sticker", old_sticker.to_string());
        local_var_form = local_var_form.text("sticker", sticker.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostReplaceStickerInSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostReplaceStickerInSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
    async fn post_restrict_chat_member<'chat_id, 'user_id, 'permissions, 'use_independent_chat_permissions, 'until_date>(&self, chat_id: models::models::BotCommandScopeChatChatId, user_id: i32, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>, until_date: Option<i32>) -> Result<ResponseContent<PostRestrictChatMemberSuccess>, Error<PostRestrictChatMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/restrictChatMember", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("permissions", permissions.to_string());
        if let Some(local_var_param_value) = use_independent_chat_permissions {
            local_var_form_params.insert("use_independent_chat_permissions", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = until_date {
            local_var_form_params.insert("until_date", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostRestrictChatMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostRestrictChatMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn post_revoke_chat_invite_link<'chat_id, 'invite_link>(&self, chat_id: models::models::RevokeChatInviteLinkRequestChatId, invite_link: &'invite_link str) -> Result<ResponseContent<PostRevokeChatInviteLinkSuccess>, Error<PostRevokeChatInviteLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/revokeChatInviteLink", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("invite_link", invite_link.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostRevokeChatInviteLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostRevokeChatInviteLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
    async fn post_save_prepared_inline_message<'user_id, 'result, 'allow_user_chats, 'allow_bot_chats, 'allow_group_chats, 'allow_channel_chats>(&self, user_id: i32, result: models::models::InlineQueryResult, allow_user_chats: Option<bool>, allow_bot_chats: Option<bool>, allow_group_chats: Option<bool>, allow_channel_chats: Option<bool>) -> Result<ResponseContent<PostSavePreparedInlineMessageSuccess>, Error<PostSavePreparedInlineMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/savePreparedInlineMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("result", result.to_string());
        if let Some(local_var_param_value) = allow_user_chats {
            local_var_form_params.insert("allow_user_chats", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_bot_chats {
            local_var_form_params.insert("allow_bot_chats", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_group_chats {
            local_var_form_params.insert("allow_group_chats", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_channel_chats {
            local_var_form_params.insert("allow_channel_chats", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSavePreparedInlineMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSavePreparedInlineMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_animation<'chat_id, 'animation, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, animation: Option<&'animation str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<&'thumbnail str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendAnimationSuccess>, Error<PostSendAnimationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendAnimation", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match animation {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("animation", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("animation", ""); },
        }
        if let Some(local_var_param_value) = duration {
            local_var_form = local_var_form.text("duration", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = width {
            local_var_form = local_var_form.text("width", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = height {
            local_var_form = local_var_form.text("height", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = thumbnail {
            local_var_form = local_var_form.text("thumbnail", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = show_caption_above_media {
            local_var_form = local_var_form.text("show_caption_above_media", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = has_spoiler {
            local_var_form = local_var_form.text("has_spoiler", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendAnimationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendAnimationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.  For sending voice messages, use the [sendVoice](https://core.telegram.org/bots/api/#sendvoice) method instead.
    async fn post_send_audio<'chat_id, 'audio, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'performer, 'title, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, audio: Option<&'audio str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, performer: Option<&'performer str>, title: Option<&'title str>, thumbnail: Option<&'thumbnail str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendAudioSuccess>, Error<PostSendAudioError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendAudio", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match audio {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("audio", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("audio", ""); },
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = duration {
            local_var_form = local_var_form.text("duration", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = performer {
            local_var_form = local_var_form.text("performer", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = title {
            local_var_form = local_var_form.text("title", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = thumbnail {
            local_var_form = local_var_form.text("thumbnail", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendAudioSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendAudioError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.  Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\\_photo*. The user will see a “sending photo” status for the bot.  We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    async fn post_send_chat_action<'chat_id, 'action, 'business_connection_id, 'message_thread_id>(&self, chat_id: models::models::SendMessageRequestChatId, action: &'action str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>) -> Result<ResponseContent<PostSendChatActionSuccess>, Error<PostSendChatActionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendChatAction", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("action", action.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendChatActionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendChatActionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send phone contacts. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_contact<'chat_id, 'phone_number, 'first_name, 'business_connection_id, 'message_thread_id, 'last_name, 'vcard, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, phone_number: &'phone_number str, first_name: &'first_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, last_name: Option<&'last_name str>, vcard: Option<&'vcard str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendContactSuccess>, Error<PostSendContactError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendContact", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("phone_number", phone_number.to_string());
        local_var_form_params.insert("first_name", first_name.to_string());
        if let Some(local_var_param_value) = last_name {
            local_var_form_params.insert("last_name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = vcard {
            local_var_form_params.insert("vcard", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendContactSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendContactError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_dice<'chat_id, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendDiceSuccess>, Error<PostSendDiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendDice", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = emoji {
            local_var_form_params.insert("emoji", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendDiceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendDiceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send general files. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_document<'chat_id, 'document, 'business_connection_id, 'message_thread_id, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'disable_content_type_detection, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, document: Option<&'document str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, thumbnail: Option<&'thumbnail str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, disable_content_type_detection: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendDocumentSuccess>, Error<PostSendDocumentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendDocument", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match document {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("document", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("document", ""); },
        }
        if let Some(local_var_param_value) = thumbnail {
            local_var_form = local_var_form.text("thumbnail", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = disable_content_type_detection {
            local_var_form = local_var_form.text("disable_content_type_detection", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendDocumentSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendDocumentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_game<'chat_id, 'game_short_name, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: i32, game_short_name: &'game_short_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostSendGameSuccess>, Error<PostSendGameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendGame", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("game_short_name", game_short_name.to_string());
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendGameSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendGameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.
    async fn post_send_gift<'gift_id, 'user_id, 'chat_id, 'pay_for_upgrade, 'text, 'text_parse_mode, 'text_entities>(&self, gift_id: &'gift_id str, user_id: Option<i32>, chat_id: Option<models::models::SendGiftRequestChatId>, pay_for_upgrade: Option<bool>, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<PostSendGiftSuccess>, Error<PostSendGiftError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendGift", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = user_id {
            local_var_form_params.insert("user_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("gift_id", gift_id.to_string());
        if let Some(local_var_param_value) = pay_for_upgrade {
            local_var_form_params.insert("pay_for_upgrade", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = text {
            local_var_form_params.insert("text", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = text_parse_mode {
            local_var_form_params.insert("text_parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = text_entities {
            local_var_form_params.insert("text_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendGiftSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendGiftError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_invoice<'chat_id, 'title, 'description, 'payload, 'currency, 'prices, 'message_thread_id, 'provider_token, 'max_tip_amount, 'suggested_tip_amounts, 'start_parameter, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, message_thread_id: Option<i32>, provider_token: Option<&'provider_token str>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, start_parameter: Option<&'start_parameter str>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostSendInvoiceSuccess>, Error<PostSendInvoiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendInvoice", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("title", title.to_string());
        local_var_form_params.insert("description", description.to_string());
        local_var_form_params.insert("payload", payload.to_string());
        if let Some(local_var_param_value) = provider_token {
            local_var_form_params.insert("provider_token", local_var_param_value.to_string());
        }
        local_var_form_params.insert("currency", currency.to_string());
        local_var_form_params.insert("prices", prices.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = max_tip_amount {
            local_var_form_params.insert("max_tip_amount", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = suggested_tip_amounts {
            local_var_form_params.insert("suggested_tip_amounts", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = start_parameter {
            local_var_form_params.insert("start_parameter", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = provider_data {
            local_var_form_params.insert("provider_data", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_url {
            local_var_form_params.insert("photo_url", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_size {
            local_var_form_params.insert("photo_size", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_width {
            local_var_form_params.insert("photo_width", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = photo_height {
            local_var_form_params.insert("photo_height", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_name {
            local_var_form_params.insert("need_name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_phone_number {
            local_var_form_params.insert("need_phone_number", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_email {
            local_var_form_params.insert("need_email", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = need_shipping_address {
            local_var_form_params.insert("need_shipping_address", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = send_phone_number_to_provider {
            local_var_form_params.insert("send_phone_number_to_provider", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = send_email_to_provider {
            local_var_form_params.insert("send_email_to_provider", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = is_flexible {
            local_var_form_params.insert("is_flexible", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendInvoiceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendInvoiceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_location<'chat_id, 'latitude, 'longitude, 'business_connection_id, 'message_thread_id, 'horizontal_accuracy, 'live_period, 'heading, 'proximity_alert_radius, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, horizontal_accuracy: Option<f64>, live_period: Option<i32>, heading: Option<i32>, proximity_alert_radius: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendLocationSuccess>, Error<PostSendLocationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendLocation", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("latitude", latitude.to_string());
        local_var_form_params.insert("longitude", longitude.to_string());
        if let Some(local_var_param_value) = horizontal_accuracy {
            local_var_form_params.insert("horizontal_accuracy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = live_period {
            local_var_form_params.insert("live_period", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = heading {
            local_var_form_params.insert("heading", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = proximity_alert_radius {
            local_var_form_params.insert("proximity_alert_radius", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendLocationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendLocationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
    async fn post_send_media_group<'chat_id, 'media, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters>(&self, chat_id: models::models::SendMessageRequestChatId, media: Vec<models::SendMediaGroupRequestMediaInner>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>) -> Result<ResponseContent<PostSendMediaGroupSuccess>, Error<PostSendMediaGroupError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendMediaGroup", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("media", media.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendMediaGroupSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendMediaGroupError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_message<'chat_id, 'text, 'business_connection_id, 'message_thread_id, 'parse_mode, 'entities, 'link_preview_options, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, text: &'text str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendMessageSuccess>, Error<PostSendMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("text", text.to_string());
        if let Some(local_var_param_value) = parse_mode {
            local_var_form_params.insert("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = entities {
            local_var_form_params.insert("entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = link_preview_options {
            local_var_form_params.insert("link_preview_options", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_paid_media<'chat_id, 'star_count, 'media, 'business_connection_id, 'payload, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendPaidMediaRequestChatId, star_count: i32, media: Vec<models::InputPaidMedia>, business_connection_id: Option<&'business_connection_id str>, payload: Option<&'payload str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendPaidMediaSuccess>, Error<PostSendPaidMediaError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendPaidMedia", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        local_var_form = local_var_form.text("star_count", star_count.to_string());
        local_var_form = local_var_form.text("media", media.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = payload {
            local_var_form = local_var_form.text("payload", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = show_caption_above_media {
            local_var_form = local_var_form.text("show_caption_above_media", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendPaidMediaSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendPaidMediaError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send photos. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_photo<'chat_id, 'photo, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, photo: Option<&'photo str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendPhotoSuccess>, Error<PostSendPhotoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendPhoto", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match photo {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("photo", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("photo", ""); },
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = show_caption_above_media {
            local_var_form = local_var_form.text("show_caption_above_media", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = has_spoiler {
            local_var_form = local_var_form.text("has_spoiler", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendPhotoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendPhotoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_poll<'chat_id, 'question, 'options, 'business_connection_id, 'message_thread_id, 'question_parse_mode, 'question_entities, 'is_anonymous, 'r_type, 'allows_multiple_answers, 'correct_option_id, 'explanation, 'explanation_parse_mode, 'explanation_entities, 'open_period, 'close_date, 'is_closed, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, question: &'question str, options: Vec<models::InputPollOption>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, question_parse_mode: Option<&'question_parse_mode str>, question_entities: Option<Vec<models::MessageEntity>>, is_anonymous: Option<bool>, r#type: Option<&'r_type str>, allows_multiple_answers: Option<bool>, correct_option_id: Option<i32>, explanation: Option<&'explanation str>, explanation_parse_mode: Option<&'explanation_parse_mode str>, explanation_entities: Option<Vec<models::MessageEntity>>, open_period: Option<i32>, close_date: Option<i32>, is_closed: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendPollSuccess>, Error<PostSendPollError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendPoll", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("question", question.to_string());
        if let Some(local_var_param_value) = question_parse_mode {
            local_var_form_params.insert("question_parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = question_entities {
            local_var_form_params.insert("question_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        local_var_form_params.insert("options", options.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = is_anonymous {
            local_var_form_params.insert("is_anonymous", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = r#type {
            local_var_form_params.insert("type", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allows_multiple_answers {
            local_var_form_params.insert("allows_multiple_answers", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = correct_option_id {
            local_var_form_params.insert("correct_option_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = explanation {
            local_var_form_params.insert("explanation", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = explanation_parse_mode {
            local_var_form_params.insert("explanation_parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = explanation_entities {
            local_var_form_params.insert("explanation_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = open_period {
            local_var_form_params.insert("open_period", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = close_date {
            local_var_form_params.insert("close_date", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = is_closed {
            local_var_form_params.insert("is_closed", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendPollSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendPollError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_sticker<'chat_id, 'sticker, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, sticker: Option<&'sticker str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendStickerSuccess>, Error<PostSendStickerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendSticker", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match sticker {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("sticker", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("sticker", ""); },
        }
        if let Some(local_var_param_value) = emoji {
            local_var_form = local_var_form.text("emoji", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendStickerSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendStickerError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_venue<'chat_id, 'latitude, 'longitude, 'title, 'address, 'business_connection_id, 'message_thread_id, 'foursquare_id, 'foursquare_type, 'google_place_id, 'google_place_type, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, latitude: f64, longitude: f64, title: &'title str, address: &'address str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, foursquare_id: Option<&'foursquare_id str>, foursquare_type: Option<&'foursquare_type str>, google_place_id: Option<&'google_place_id str>, google_place_type: Option<&'google_place_type str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVenueSuccess>, Error<PostSendVenueError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendVenue", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form_params.insert("message_thread_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("latitude", latitude.to_string());
        local_var_form_params.insert("longitude", longitude.to_string());
        local_var_form_params.insert("title", title.to_string());
        local_var_form_params.insert("address", address.to_string());
        if let Some(local_var_param_value) = foursquare_id {
            local_var_form_params.insert("foursquare_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = foursquare_type {
            local_var_form_params.insert("foursquare_type", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = google_place_id {
            local_var_form_params.insert("google_place_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = google_place_type {
            local_var_form_params.insert("google_place_type", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form_params.insert("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form_params.insert("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form_params.insert("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form_params.insert("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form_params.insert("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendVenueSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendVenueError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_video<'chat_id, 'video, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'cover, 'start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'supports_streaming, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, video: Option<&'video str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<&'thumbnail str>, cover: Option<&'cover str>, start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, supports_streaming: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVideoSuccess>, Error<PostSendVideoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendVideo", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match video {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("video", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("video", ""); },
        }
        if let Some(local_var_param_value) = duration {
            local_var_form = local_var_form.text("duration", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = width {
            local_var_form = local_var_form.text("width", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = height {
            local_var_form = local_var_form.text("height", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = thumbnail {
            local_var_form = local_var_form.text("thumbnail", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = cover {
            local_var_form = local_var_form.text("cover", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = start_timestamp {
            local_var_form = local_var_form.text("start_timestamp", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = show_caption_above_media {
            local_var_form = local_var_form.text("show_caption_above_media", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = has_spoiler {
            local_var_form = local_var_form.text("has_spoiler", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = supports_streaming {
            local_var_form = local_var_form.text("supports_streaming", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendVideoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendVideoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn post_send_video_note<'chat_id, 'video_note, 'business_connection_id, 'message_thread_id, 'duration, 'length, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, video_note: Option<&'video_note str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, length: Option<i32>, thumbnail: Option<&'thumbnail str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVideoNoteSuccess>, Error<PostSendVideoNoteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendVideoNote", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match video_note {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("video_note", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("video_note", ""); },
        }
        if let Some(local_var_param_value) = duration {
            local_var_form = local_var_form.text("duration", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = length {
            local_var_form = local_var_form.text("length", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = thumbnail {
            local_var_form = local_var_form.text("thumbnail", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendVideoNoteSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendVideoNoteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as [Audio](https://core.telegram.org/bots/api/#audio) or [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    async fn post_send_voice<'chat_id, 'voice, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, voice: Option<&'voice str>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessageRequestReplyMarkup>) -> Result<ResponseContent<PostSendVoiceSuccess>, Error<PostSendVoiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sendVoice", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form = local_var_form.text("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_thread_id {
            local_var_form = local_var_form.text("message_thread_id", local_var_param_value.to_string());
        }
        match voice {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("voice", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("voice", ""); },
        }
        if let Some(local_var_param_value) = caption {
            local_var_form = local_var_form.text("caption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = parse_mode {
            local_var_form = local_var_form.text("parse_mode", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = caption_entities {
            local_var_form = local_var_form.text("caption_entities", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = duration {
            local_var_form = local_var_form.text("duration", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_notification {
            local_var_form = local_var_form.text("disable_notification", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = protect_content {
            local_var_form = local_var_form.text("protect_content", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allow_paid_broadcast {
            local_var_form = local_var_form.text("allow_paid_broadcast", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_effect_id {
            local_var_form = local_var_form.text("message_effect_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_parameters {
            local_var_form = local_var_form.text("reply_parameters", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form = local_var_form.text("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSendVoiceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSendVoiceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the bio of a managed business account. Requires the *can\\_change\\_bio* business bot right. Returns *True* on success.
    async fn post_set_business_account_bio<'business_connection_id, 'bio>(&self, business_connection_id: &'business_connection_id str, bio: Option<&'bio str>) -> Result<ResponseContent<PostSetBusinessAccountBioSuccess>, Error<PostSetBusinessAccountBioError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setBusinessAccountBio", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        if let Some(local_var_param_value) = bio {
            local_var_form_params.insert("bio", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetBusinessAccountBioSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetBusinessAccountBioError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the *can\\_change\\_gift\\_settings* business bot right. Returns *True* on success.
    async fn post_set_business_account_gift_settings<'business_connection_id, 'show_gift_button, 'accepted_gift_types>(&self, business_connection_id: &'business_connection_id str, show_gift_button: bool, accepted_gift_types: models::models::AcceptedGiftTypes) -> Result<ResponseContent<PostSetBusinessAccountGiftSettingsSuccess>, Error<PostSetBusinessAccountGiftSettingsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setBusinessAccountGiftSettings", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("show_gift_button", show_gift_button.to_string());
        local_var_form_params.insert("accepted_gift_types", accepted_gift_types.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetBusinessAccountGiftSettingsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetBusinessAccountGiftSettingsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the first and last name of a managed business account. Requires the *can\\_change\\_name* business bot right. Returns *True* on success.
    async fn post_set_business_account_name<'business_connection_id, 'first_name, 'last_name>(&self, business_connection_id: &'business_connection_id str, first_name: &'first_name str, last_name: Option<&'last_name str>) -> Result<ResponseContent<PostSetBusinessAccountNameSuccess>, Error<PostSetBusinessAccountNameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setBusinessAccountName", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("first_name", first_name.to_string());
        if let Some(local_var_param_value) = last_name {
            local_var_form_params.insert("last_name", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetBusinessAccountNameSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetBusinessAccountNameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn post_set_business_account_profile_photo<'business_connection_id, 'photo, 'is_public>(&self, business_connection_id: &'business_connection_id str, photo: models::models::InputProfilePhoto, is_public: Option<bool>) -> Result<ResponseContent<PostSetBusinessAccountProfilePhotoSuccess>, Error<PostSetBusinessAccountProfilePhotoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setBusinessAccountProfilePhoto", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("business_connection_id", business_connection_id.to_string());
        local_var_form = local_var_form.text("photo", photo.to_string());
        if let Some(local_var_param_value) = is_public {
            local_var_form = local_var_form.text("is_public", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetBusinessAccountProfilePhotoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetBusinessAccountProfilePhotoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the username of a managed business account. Requires the *can\\_change\\_username* business bot right. Returns *True* on success.
    async fn post_set_business_account_username<'business_connection_id, 'username>(&self, business_connection_id: &'business_connection_id str, username: Option<&'username str>) -> Result<ResponseContent<PostSetBusinessAccountUsernameSuccess>, Error<PostSetBusinessAccountUsernameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setBusinessAccountUsername", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        if let Some(local_var_param_value) = username {
            local_var_form_params.insert("username", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetBusinessAccountUsernameSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetBusinessAccountUsernameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
    async fn post_set_chat_administrator_custom_title<'chat_id, 'user_id, 'custom_title>(&self, chat_id: models::models::BotCommandScopeChatChatId, user_id: i32, custom_title: &'custom_title str) -> Result<ResponseContent<PostSetChatAdministratorCustomTitleSuccess>, Error<PostSetChatAdministratorCustomTitleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatAdministratorCustomTitle", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("custom_title", custom_title.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatAdministratorCustomTitleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatAdministratorCustomTitleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_set_chat_description<'chat_id, 'description>(&self, chat_id: models::models::SendMessageRequestChatId, description: Option<&'description str>) -> Result<ResponseContent<PostSetChatDescriptionSuccess>, Error<PostSetChatDescriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatDescription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = description {
            local_var_form_params.insert("description", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatDescriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatDescriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
    async fn post_set_chat_menu_button<'chat_id, 'menu_button>(&self, chat_id: Option<i32>, menu_button: Option<models::models::MenuButton>) -> Result<ResponseContent<PostSetChatMenuButtonSuccess>, Error<PostSetChatMenuButtonError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatMenuButton", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = menu_button {
            local_var_form_params.insert("menu_button", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatMenuButtonSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatMenuButtonError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\\_restrict\\_members* administrator rights. Returns *True* on success.
    async fn post_set_chat_permissions<'chat_id, 'permissions, 'use_independent_chat_permissions>(&self, chat_id: models::models::BotCommandScopeChatChatId, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>) -> Result<ResponseContent<PostSetChatPermissionsSuccess>, Error<PostSetChatPermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatPermissions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("permissions", permissions.to_string());
        if let Some(local_var_param_value) = use_independent_chat_permissions {
            local_var_form_params.insert("use_independent_chat_permissions", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatPermissionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatPermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_set_chat_photo<'chat_id, 'photo>(&self, chat_id: models::models::SendMessageRequestChatId, photo: Option<models::serde_json::Value>) -> Result<ResponseContent<PostSetChatPhotoSuccess>, Error<PostSetChatPhotoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatPhoto", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("chat_id", chat_id.to_string());
        match photo {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("photo", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("photo", ""); },
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatPhotoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatPhotoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn post_set_chat_sticker_set<'chat_id, 'sticker_set_name>(&self, chat_id: models::models::BotCommandScopeChatChatId, sticker_set_name: &'sticker_set_name str) -> Result<ResponseContent<PostSetChatStickerSetSuccess>, Error<PostSetChatStickerSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatStickerSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("sticker_set_name", sticker_set_name.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatStickerSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatStickerSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_set_chat_title<'chat_id, 'title>(&self, chat_id: models::models::SendMessageRequestChatId, title: &'title str) -> Result<ResponseContent<PostSetChatTitleSuccess>, Error<PostSetChatTitleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setChatTitle", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("title", title.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetChatTitleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetChatTitleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
    async fn post_set_custom_emoji_sticker_set_thumbnail<'name, 'custom_emoji_id>(&self, name: &'name str, custom_emoji_id: Option<&'custom_emoji_id str>) -> Result<ResponseContent<PostSetCustomEmojiStickerSetThumbnailSuccess>, Error<PostSetCustomEmojiStickerSetThumbnailError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setCustomEmojiStickerSetThumbnail", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("name", name.to_string());
        if let Some(local_var_param_value) = custom_emoji_id {
            local_var_form_params.insert("custom_emoji_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetCustomEmojiStickerSetThumbnailSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetCustomEmojiStickerSetThumbnailError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
    async fn post_set_game_score<'user_id, 'score, 'force, 'disable_edit_message, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, score: i32, force: Option<bool>, disable_edit_message: Option<bool>, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<PostSetGameScoreSuccess>, Error<PostSetGameScoreError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setGameScore", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("score", score.to_string());
        if let Some(local_var_param_value) = force {
            local_var_form_params.insert("force", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = disable_edit_message {
            local_var_form_params.insert("disable_edit_message", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetGameScoreSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetGameScoreError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.
    async fn post_set_message_reaction<'chat_id, 'message_id, 'reaction, 'is_big>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32, reaction: Option<Vec<models::ReactionType>>, is_big: Option<bool>) -> Result<ResponseContent<PostSetMessageReactionSuccess>, Error<PostSetMessageReactionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setMessageReaction", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_id", message_id.to_string());
        if let Some(local_var_param_value) = reaction {
            local_var_form_params.insert("reaction", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = is_big {
            local_var_form_params.insert("is_big", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetMessageReactionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetMessageReactionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
    async fn post_set_my_commands<'commands, 'scope, 'language_code>(&self, commands: Vec<models::BotCommand>, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyCommandsSuccess>, Error<PostSetMyCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setMyCommands", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("commands", commands.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        if let Some(local_var_param_value) = scope {
            local_var_form_params.insert("scope", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetMyCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetMyCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
    async fn post_set_my_default_administrator_rights<'rights, 'for_channels>(&self, rights: Option<models::models::ChatAdministratorRights>, for_channels: Option<bool>) -> Result<ResponseContent<PostSetMyDefaultAdministratorRightsSuccess>, Error<PostSetMyDefaultAdministratorRightsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setMyDefaultAdministratorRights", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = rights {
            local_var_form_params.insert("rights", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = for_channels {
            local_var_form_params.insert("for_channels", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetMyDefaultAdministratorRightsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetMyDefaultAdministratorRightsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
    async fn post_set_my_description<'description, 'language_code>(&self, description: Option<&'description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyDescriptionSuccess>, Error<PostSetMyDescriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setMyDescription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = description {
            local_var_form_params.insert("description", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetMyDescriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetMyDescriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's name. Returns *True* on success.
    async fn post_set_my_name<'name, 'language_code>(&self, name: Option<&'name str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyNameSuccess>, Error<PostSetMyNameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setMyName", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = name {
            local_var_form_params.insert("name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetMyNameSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetMyNameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
    async fn post_set_my_short_description<'short_description, 'language_code>(&self, short_description: Option<&'short_description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<PostSetMyShortDescriptionSuccess>, Error<PostSetMyShortDescriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setMyShortDescription", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = short_description {
            local_var_form_params.insert("short_description", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language_code {
            local_var_form_params.insert("language_code", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetMyShortDescriptionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetMyShortDescriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.  Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    async fn post_set_passport_data_errors<'user_id, 'errors>(&self, user_id: i32, errors: Vec<models::PassportElementError>) -> Result<ResponseContent<PostSetPassportDataErrorsSuccess>, Error<PostSetPassportDataErrorsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setPassportDataErrors", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        local_var_form_params.insert("errors", errors.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetPassportDataErrorsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetPassportDataErrorsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn post_set_sticker_emoji_list<'sticker, 'emoji_list>(&self, sticker: &'sticker str, emoji_list: Vec<String>) -> Result<ResponseContent<PostSetStickerEmojiListSuccess>, Error<PostSetStickerEmojiListError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setStickerEmojiList", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("sticker", sticker.to_string());
        local_var_form_params.insert("emoji_list", emoji_list.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetStickerEmojiListSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetStickerEmojiListError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn post_set_sticker_keywords<'sticker, 'keywords>(&self, sticker: &'sticker str, keywords: Option<Vec<String>>) -> Result<ResponseContent<PostSetStickerKeywordsSuccess>, Error<PostSetStickerKeywordsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setStickerKeywords", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("sticker", sticker.to_string());
        if let Some(local_var_param_value) = keywords {
            local_var_form_params.insert("keywords", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetStickerKeywordsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetStickerKeywordsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
    async fn post_set_sticker_mask_position<'sticker, 'mask_position>(&self, sticker: &'sticker str, mask_position: Option<models::models::MaskPosition>) -> Result<ResponseContent<PostSetStickerMaskPositionSuccess>, Error<PostSetStickerMaskPositionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setStickerMaskPosition", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("sticker", sticker.to_string());
        if let Some(local_var_param_value) = mask_position {
            local_var_form_params.insert("mask_position", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetStickerMaskPositionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetStickerMaskPositionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
    async fn post_set_sticker_position_in_set<'sticker, 'position>(&self, sticker: &'sticker str, position: i32) -> Result<ResponseContent<PostSetStickerPositionInSetSuccess>, Error<PostSetStickerPositionInSetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setStickerPositionInSet", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("sticker", sticker.to_string());
        local_var_form_params.insert("position", position.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetStickerPositionInSetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetStickerPositionInSetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
    async fn post_set_sticker_set_thumbnail<'name, 'user_id, 'format, 'thumbnail>(&self, name: &'name str, user_id: i32, format: &'format str, thumbnail: Option<&'thumbnail str>) -> Result<ResponseContent<PostSetStickerSetThumbnailSuccess>, Error<PostSetStickerSetThumbnailError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setStickerSetThumbnail", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("name", name.to_string());
        local_var_form = local_var_form.text("user_id", user_id.to_string());
        if let Some(local_var_param_value) = thumbnail {
            local_var_form = local_var_form.text("thumbnail", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("format", format.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetStickerSetThumbnailSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetStickerSetThumbnailError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the title of a created sticker set. Returns *True* on success.
    async fn post_set_sticker_set_title<'name, 'title>(&self, name: &'name str, title: &'title str) -> Result<ResponseContent<PostSetStickerSetTitleSuccess>, Error<PostSetStickerSetTitleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setStickerSetTitle", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("name", name.to_string());
        local_var_form_params.insert("title", title.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetStickerSetTitleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetStickerSetTitleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
    async fn post_set_user_emoji_status<'user_id, 'emoji_status_custom_emoji_id, 'emoji_status_expiration_date>(&self, user_id: i32, emoji_status_custom_emoji_id: Option<&'emoji_status_custom_emoji_id str>, emoji_status_expiration_date: Option<i32>) -> Result<ResponseContent<PostSetUserEmojiStatusSuccess>, Error<PostSetUserEmojiStatusError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setUserEmojiStatus", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = emoji_status_custom_emoji_id {
            local_var_form_params.insert("emoji_status_custom_emoji_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = emoji_status_expiration_date {
            local_var_form_params.insert("emoji_status_expiration_date", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetUserEmojiStatusSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetUserEmojiStatusError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts. Returns *True* on success.  If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    async fn post_set_webhook<'url, 'certificate, 'ip_address, 'max_connections, 'allowed_updates, 'drop_pending_updates, 'secret_token>(&self, url: &'url str, certificate: Option<models::serde_json::Value>, ip_address: Option<&'ip_address str>, max_connections: Option<i32>, allowed_updates: Option<Vec<String>>, drop_pending_updates: Option<bool>, secret_token: Option<&'secret_token str>) -> Result<ResponseContent<PostSetWebhookSuccess>, Error<PostSetWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/setWebhook", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("url", url.to_string());
        if let Some(local_var_param_value) = certificate {
            local_var_form = local_var_form.text("certificate", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = ip_address {
            local_var_form = local_var_form.text("ip_address", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = max_connections {
            local_var_form = local_var_form.text("max_connections", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = allowed_updates {
            local_var_form = local_var_form.text("allowed_updates", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
        }
        if let Some(local_var_param_value) = drop_pending_updates {
            local_var_form = local_var_form.text("drop_pending_updates", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = secret_token {
            local_var_form = local_var_form.text("secret_token", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostSetWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostSetWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to stop updating a live location message before *live\\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn post_stop_message_live_location<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostStopMessageLiveLocationSuccess>, Error<PostStopMessageLiveLocationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stopMessageLiveLocation", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = chat_id {
            local_var_form_params.insert("chat_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = inline_message_id {
            local_var_form_params.insert("inline_message_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostStopMessageLiveLocationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostStopMessageLiveLocationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
    async fn post_stop_poll<'chat_id, 'message_id, 'business_connection_id, 'reply_markup>(&self, chat_id: models::models::SendMessageRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<PostStopPollSuccess>, Error<PostStopPollError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stopPoll", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_id", message_id.to_string());
        if let Some(local_var_param_value) = reply_markup {
            local_var_form_params.insert("reply_markup", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostStopPollSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostStopPollError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the *can\\_transfer\\_stars* business bot right. Returns *True* on success.
    async fn post_transfer_business_account_stars<'business_connection_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, star_count: i32) -> Result<ResponseContent<PostTransferBusinessAccountStarsSuccess>, Error<PostTransferBusinessAccountStarsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/transferBusinessAccountStars", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("star_count", star_count.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostTransferBusinessAccountStarsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostTransferBusinessAccountStarsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Transfers an owned unique gift to another user. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Requires *can\\_transfer\\_stars* business bot right if the transfer is paid. Returns *True* on success.
    async fn post_transfer_gift<'business_connection_id, 'owned_gift_id, 'new_owner_chat_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, new_owner_chat_id: i32, star_count: Option<i32>) -> Result<ResponseContent<PostTransferGiftSuccess>, Error<PostTransferGiftError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/transferGift", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("owned_gift_id", owned_gift_id.to_string());
        local_var_form_params.insert("new_owner_chat_id", new_owner_chat_id.to_string());
        if let Some(local_var_param_value) = star_count {
            local_var_form_params.insert("star_count", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostTransferGiftSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostTransferGiftError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\\_if\\_banned*. Returns *True* on success.
    async fn post_unban_chat_member<'chat_id, 'user_id, 'only_if_banned>(&self, chat_id: models::models::BanChatMemberRequestChatId, user_id: i32, only_if_banned: Option<bool>) -> Result<ResponseContent<PostUnbanChatMemberSuccess>, Error<PostUnbanChatMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unbanChatMember", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = only_if_banned {
            local_var_form_params.insert("only_if_banned", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnbanChatMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnbanChatMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn post_unban_chat_sender_chat<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessageRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<PostUnbanChatSenderChatSuccess>, Error<PostUnbanChatSenderChatError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unbanChatSenderChat", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("sender_chat_id", sender_chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnbanChatSenderChatSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnbanChatSenderChatError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn post_unhide_general_forum_topic<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostUnhideGeneralForumTopicSuccess>, Error<PostUnhideGeneralForumTopicError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unhideGeneralForumTopic", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnhideGeneralForumTopicSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnhideGeneralForumTopicError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn post_unpin_all_chat_messages<'chat_id>(&self, chat_id: models::models::SendMessageRequestChatId) -> Result<ResponseContent<PostUnpinAllChatMessagesSuccess>, Error<PostUnpinAllChatMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unpinAllChatMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnpinAllChatMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnpinAllChatMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn post_unpin_all_forum_topic_messages<'chat_id, 'message_thread_id>(&self, chat_id: models::models::BotCommandScopeChatChatId, message_thread_id: i32) -> Result<ResponseContent<PostUnpinAllForumTopicMessagesSuccess>, Error<PostUnpinAllForumTopicMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unpinAllForumTopicMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_form_params.insert("message_thread_id", message_thread_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnpinAllForumTopicMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnpinAllForumTopicMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn post_unpin_all_general_forum_topic_messages<'chat_id>(&self, chat_id: models::models::BotCommandScopeChatChatId) -> Result<ResponseContent<PostUnpinAllGeneralForumTopicMessagesSuccess>, Error<PostUnpinAllGeneralForumTopicMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unpinAllGeneralForumTopicMessages", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnpinAllGeneralForumTopicMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnpinAllGeneralForumTopicMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn post_unpin_chat_message<'chat_id, 'business_connection_id, 'message_id>(&self, chat_id: models::models::SendMessageRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_id: Option<i32>) -> Result<ResponseContent<PostUnpinChatMessageSuccess>, Error<PostUnpinChatMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/unpinChatMessage", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = business_connection_id {
            local_var_form_params.insert("business_connection_id", local_var_param_value.to_string());
        }
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = message_id {
            local_var_form_params.insert("message_id", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUnpinChatMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUnpinChatMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Upgrades a given regular gift to a unique gift. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Additionally requires the *can\\_transfer\\_stars* business bot right if the upgrade is paid. Returns *True* on success.
    async fn post_upgrade_gift<'business_connection_id, 'owned_gift_id, 'keep_original_details, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, keep_original_details: Option<bool>, star_count: Option<i32>) -> Result<ResponseContent<PostUpgradeGiftSuccess>, Error<PostUpgradeGiftError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/upgradeGift", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("business_connection_id", business_connection_id.to_string());
        local_var_form_params.insert("owned_gift_id", owned_gift_id.to_string());
        if let Some(local_var_param_value) = keep_original_details {
            local_var_form_params.insert("keep_original_details", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = star_count {
            local_var_form_params.insert("star_count", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUpgradeGiftSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUpgradeGiftError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
    async fn post_upload_sticker_file<'user_id, 'sticker, 'sticker_format>(&self, user_id: i32, sticker: Option<models::serde_json::Value>, sticker_format: &'sticker_format str) -> Result<ResponseContent<PostUploadStickerFileSuccess>, Error<PostUploadStickerFileError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/uploadStickerFile", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("user_id", user_id.to_string());
        match sticker {
            Some(local_var_param_value) => { local_var_form = local_var_form.text("sticker", local_var_param_value.to_string()); },
            None => { local_var_form = local_var_form.text("sticker", ""); },
        }
        local_var_form = local_var_form.text("sticker_format", sticker_format.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostUploadStickerFileSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostUploadStickerFileError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn post_verify_chat<'chat_id, 'custom_description>(&self, chat_id: models::models::SendMessageRequestChatId, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<PostVerifyChatSuccess>, Error<PostVerifyChatError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/verifyChat", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("chat_id", chat_id.to_string());
        if let Some(local_var_param_value) = custom_description {
            local_var_form_params.insert("custom_description", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostVerifyChatSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostVerifyChatError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn post_verify_user<'user_id, 'custom_description>(&self, user_id: i32, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<PostVerifyUserSuccess>, Error<PostVerifyUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/verifyUser", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        local_var_form_params.insert("user_id", user_id.to_string());
        if let Some(local_var_param_value) = custom_description {
            local_var_form_params.insert("custom_description", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PostVerifyUserSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostVerifyUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed successes of method [`post_add_sticker_to_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAddStickerToSetSuccess {
    Status200(models::AddStickerToSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_answer_callback_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerCallbackQuerySuccess {
    Status200(models::AnswerCallbackQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_answer_inline_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerInlineQuerySuccess {
    Status200(models::AnswerInlineQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_answer_pre_checkout_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerPreCheckoutQuerySuccess {
    Status200(models::AnswerPreCheckoutQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_answer_shipping_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerShippingQuerySuccess {
    Status200(models::AnswerShippingQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_answer_web_app_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerWebAppQuerySuccess {
    Status200(models::AnswerWebAppQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_approve_chat_join_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostApproveChatJoinRequestSuccess {
    Status200(models::ApproveChatJoinRequestResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_ban_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostBanChatMemberSuccess {
    Status200(models::BanChatMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_ban_chat_sender_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostBanChatSenderChatSuccess {
    Status200(models::BanChatSenderChatResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_close`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCloseSuccess {
    Status200(models::CloseResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_close_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCloseForumTopicSuccess {
    Status200(models::CloseForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_close_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCloseGeneralForumTopicSuccess {
    Status200(models::CloseGeneralForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_convert_gift_to_stars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostConvertGiftToStarsSuccess {
    Status200(models::ConvertGiftToStarsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_copy_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCopyMessageSuccess {
    Status200(models::CopyMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_copy_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCopyMessagesSuccess {
    Status200(models::CopyMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_create_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateChatInviteLinkSuccess {
    Status200(models::CreateChatInviteLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_create_chat_subscription_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateChatSubscriptionInviteLinkSuccess {
    Status200(models::CreateChatSubscriptionInviteLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_create_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateForumTopicSuccess {
    Status200(models::CreateForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_create_invoice_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateInvoiceLinkSuccess {
    Status200(models::CreateInvoiceLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_create_new_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateNewStickerSetSuccess {
    Status200(models::CreateNewStickerSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_decline_chat_join_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeclineChatJoinRequestSuccess {
    Status200(models::DeclineChatJoinRequestResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_business_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteBusinessMessagesSuccess {
    Status200(models::DeleteBusinessMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_chat_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteChatPhotoSuccess {
    Status200(models::DeleteChatPhotoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_chat_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteChatStickerSetSuccess {
    Status200(models::DeleteChatStickerSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteForumTopicSuccess {
    Status200(models::DeleteForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteMessageSuccess {
    Status200(models::DeleteMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteMessagesSuccess {
    Status200(models::DeleteMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteMyCommandsSuccess {
    Status200(models::DeleteMyCommandsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_sticker_from_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteStickerFromSetSuccess {
    Status200(models::DeleteStickerFromSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteStickerSetSuccess {
    Status200(models::DeleteStickerSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_story`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteStorySuccess {
    Status200(models::DeleteStoryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteWebhookSuccess {
    Status200(models::DeleteWebhookResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditChatInviteLinkSuccess {
    Status200(models::EditChatInviteLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_chat_subscription_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditChatSubscriptionInviteLinkSuccess {
    Status200(models::EditChatSubscriptionInviteLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditForumTopicSuccess {
    Status200(models::EditForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditGeneralForumTopicSuccess {
    Status200(models::EditGeneralForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_message_caption`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageCaptionSuccess {
    Status200(models::EditMessageCaptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_message_live_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageLiveLocationSuccess {
    Status200(models::EditMessageLiveLocationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_message_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageMediaSuccess {
    Status200(models::EditMessageMediaResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_message_reply_markup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageReplyMarkupSuccess {
    Status200(models::EditMessageReplyMarkupResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_message_text`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageTextSuccess {
    Status200(models::EditMessageTextResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_story`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditStorySuccess {
    Status200(models::EditStoryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_edit_user_star_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditUserStarSubscriptionSuccess {
    Status200(models::EditUserStarSubscriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_export_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostExportChatInviteLinkSuccess {
    Status200(models::ExportChatInviteLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_forward_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostForwardMessageSuccess {
    Status200(models::ForwardMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_forward_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostForwardMessagesSuccess {
    Status200(models::ForwardMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_available_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetAvailableGiftsSuccess {
    Status200(models::GetAvailableGiftsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_business_account_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetBusinessAccountGiftsSuccess {
    Status200(models::GetBusinessAccountGiftsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_business_account_star_balance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetBusinessAccountStarBalanceSuccess {
    Status200(models::GetBusinessAccountStarBalanceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_business_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetBusinessConnectionSuccess {
    Status200(models::GetBusinessConnectionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatSuccess {
    Status200(models::GetChatResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_chat_administrators`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatAdministratorsSuccess {
    Status200(models::GetChatAdministratorsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatMemberSuccess {
    Status200(models::GetChatMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_chat_member_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatMemberCountSuccess {
    Status200(models::GetChatMemberCountResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_chat_menu_button`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatMenuButtonSuccess {
    Status200(models::GetChatMenuButtonResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_custom_emoji_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetCustomEmojiStickersSuccess {
    Status200(models::GetCustomEmojiStickersResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetFileSuccess {
    Status200(models::GetFileResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_forum_topic_icon_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetForumTopicIconStickersSuccess {
    Status200(models::GetForumTopicIconStickersResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_game_high_scores`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetGameHighScoresSuccess {
    Status200(models::GetGameHighScoresResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMeSuccess {
    Status200(models::GetMeResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyCommandsSuccess {
    Status200(models::GetMyCommandsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_my_default_administrator_rights`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyDefaultAdministratorRightsSuccess {
    Status200(models::GetMyDefaultAdministratorRightsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_my_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyDescriptionSuccess {
    Status200(models::GetMyDescriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_my_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyNameSuccess {
    Status200(models::GetMyNameResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_my_short_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyShortDescriptionSuccess {
    Status200(models::GetMyShortDescriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_star_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetStarTransactionsSuccess {
    Status200(models::GetStarTransactionsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetStickerSetSuccess {
    Status200(models::GetStickerSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_updates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetUpdatesSuccess {
    Status200(models::GetUpdatesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_user_chat_boosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetUserChatBoostsSuccess {
    Status200(models::GetUserChatBoostsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_user_profile_photos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetUserProfilePhotosSuccess {
    Status200(models::GetUserProfilePhotosResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_get_webhook_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetWebhookInfoSuccess {
    Status200(models::GetWebhookInfoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_gift_premium_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGiftPremiumSubscriptionSuccess {
    Status200(models::GiftPremiumSubscriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_hide_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostHideGeneralForumTopicSuccess {
    Status200(models::HideGeneralForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_leave_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLeaveChatSuccess {
    Status200(models::LeaveChatResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_log_out`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLogOutSuccess {
    Status200(models::LogOutResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_pin_chat_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPinChatMessageSuccess {
    Status200(models::PinChatMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_post_story`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPostStorySuccess {
    Status200(models::PostStoryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_promote_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPromoteChatMemberSuccess {
    Status200(models::PromoteChatMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_read_business_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReadBusinessMessageSuccess {
    Status200(models::ReadBusinessMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_refund_star_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRefundStarPaymentSuccess {
    Status200(models::RefundStarPaymentResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_remove_business_account_profile_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRemoveBusinessAccountProfilePhotoSuccess {
    Status200(models::RemoveBusinessAccountProfilePhotoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_remove_chat_verification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRemoveChatVerificationSuccess {
    Status200(models::RemoveChatVerificationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_remove_user_verification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRemoveUserVerificationSuccess {
    Status200(models::RemoveUserVerificationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_reopen_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReopenForumTopicSuccess {
    Status200(models::ReopenForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_reopen_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReopenGeneralForumTopicSuccess {
    Status200(models::ReopenGeneralForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_replace_sticker_in_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReplaceStickerInSetSuccess {
    Status200(models::ReplaceStickerInSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_restrict_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRestrictChatMemberSuccess {
    Status200(models::RestrictChatMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_revoke_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRevokeChatInviteLinkSuccess {
    Status200(models::RevokeChatInviteLinkResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_save_prepared_inline_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSavePreparedInlineMessageSuccess {
    Status200(models::SavePreparedInlineMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_animation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendAnimationSuccess {
    Status200(models::SendAnimationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_audio`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendAudioSuccess {
    Status200(models::SendAudioResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_chat_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendChatActionSuccess {
    Status200(models::SendChatActionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendContactSuccess {
    Status200(models::SendContactResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_dice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendDiceSuccess {
    Status200(models::SendDiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_document`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendDocumentSuccess {
    Status200(models::SendDocumentResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendGameSuccess {
    Status200(models::SendGameResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_gift`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendGiftSuccess {
    Status200(models::SendGiftResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_invoice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendInvoiceSuccess {
    Status200(models::SendInvoiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendLocationSuccess {
    Status200(models::SendLocationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_media_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendMediaGroupSuccess {
    Status200(models::SendMediaGroupResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendMessageSuccess {
    Status200(models::SendMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_paid_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendPaidMediaSuccess {
    Status200(models::SendPaidMediaResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendPhotoSuccess {
    Status200(models::SendPhotoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_poll`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendPollSuccess {
    Status200(models::SendPollResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendStickerSuccess {
    Status200(models::SendStickerResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_venue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVenueSuccess {
    Status200(models::SendVenueResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_video`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVideoSuccess {
    Status200(models::SendVideoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_video_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVideoNoteSuccess {
    Status200(models::SendVideoNoteResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_send_voice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVoiceSuccess {
    Status200(models::SendVoiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_business_account_bio`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountBioSuccess {
    Status200(models::SetBusinessAccountBioResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_business_account_gift_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountGiftSettingsSuccess {
    Status200(models::SetBusinessAccountGiftSettingsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_business_account_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountNameSuccess {
    Status200(models::SetBusinessAccountNameResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_business_account_profile_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountProfilePhotoSuccess {
    Status200(models::SetBusinessAccountProfilePhotoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_business_account_username`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountUsernameSuccess {
    Status200(models::SetBusinessAccountUsernameResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_administrator_custom_title`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatAdministratorCustomTitleSuccess {
    Status200(models::SetChatAdministratorCustomTitleResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatDescriptionSuccess {
    Status200(models::SetChatDescriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_menu_button`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatMenuButtonSuccess {
    Status200(models::SetChatMenuButtonResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatPermissionsSuccess {
    Status200(models::SetChatPermissionsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatPhotoSuccess {
    Status200(models::SetChatPhotoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatStickerSetSuccess {
    Status200(models::SetChatStickerSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_chat_title`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatTitleSuccess {
    Status200(models::SetChatTitleResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_custom_emoji_sticker_set_thumbnail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetCustomEmojiStickerSetThumbnailSuccess {
    Status200(models::SetCustomEmojiStickerSetThumbnailResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_game_score`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetGameScoreSuccess {
    Status200(models::SetGameScoreResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMessageReactionSuccess {
    Status200(models::SetMessageReactionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyCommandsSuccess {
    Status200(models::SetMyCommandsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_my_default_administrator_rights`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyDefaultAdministratorRightsSuccess {
    Status200(models::SetMyDefaultAdministratorRightsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_my_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyDescriptionSuccess {
    Status200(models::SetMyDescriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_my_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyNameSuccess {
    Status200(models::SetMyNameResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_my_short_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyShortDescriptionSuccess {
    Status200(models::SetMyShortDescriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_passport_data_errors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetPassportDataErrorsSuccess {
    Status200(models::SetPassportDataErrorsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_sticker_emoji_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerEmojiListSuccess {
    Status200(models::SetStickerEmojiListResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_sticker_keywords`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerKeywordsSuccess {
    Status200(models::SetStickerKeywordsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_sticker_mask_position`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerMaskPositionSuccess {
    Status200(models::SetStickerMaskPositionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_sticker_position_in_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerPositionInSetSuccess {
    Status200(models::SetStickerPositionInSetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_sticker_set_thumbnail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerSetThumbnailSuccess {
    Status200(models::SetStickerSetThumbnailResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_sticker_set_title`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerSetTitleSuccess {
    Status200(models::SetStickerSetTitleResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_user_emoji_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetUserEmojiStatusSuccess {
    Status200(models::SetUserEmojiStatusResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_set_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetWebhookSuccess {
    Status200(models::SetWebhookResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_stop_message_live_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostStopMessageLiveLocationSuccess {
    Status200(models::StopMessageLiveLocationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_stop_poll`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostStopPollSuccess {
    Status200(models::StopPollResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_transfer_business_account_stars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostTransferBusinessAccountStarsSuccess {
    Status200(models::TransferBusinessAccountStarsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_transfer_gift`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostTransferGiftSuccess {
    Status200(models::TransferGiftResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unban_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnbanChatMemberSuccess {
    Status200(models::UnbanChatMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unban_chat_sender_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnbanChatSenderChatSuccess {
    Status200(models::UnbanChatSenderChatResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unhide_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnhideGeneralForumTopicSuccess {
    Status200(models::UnhideGeneralForumTopicResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unpin_all_chat_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinAllChatMessagesSuccess {
    Status200(models::UnpinAllChatMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unpin_all_forum_topic_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinAllForumTopicMessagesSuccess {
    Status200(models::UnpinAllForumTopicMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unpin_all_general_forum_topic_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinAllGeneralForumTopicMessagesSuccess {
    Status200(models::UnpinAllGeneralForumTopicMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_unpin_chat_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinChatMessageSuccess {
    Status200(models::UnpinChatMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_upgrade_gift`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUpgradeGiftSuccess {
    Status200(models::UpgradeGiftResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_upload_sticker_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUploadStickerFileSuccess {
    Status200(models::UploadStickerFileResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_verify_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostVerifyChatSuccess {
    Status200(models::VerifyChatResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_verify_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostVerifyUserSuccess {
    Status200(models::VerifyUserResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_add_sticker_to_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAddStickerToSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_answer_callback_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerCallbackQueryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_answer_inline_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerInlineQueryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_answer_pre_checkout_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerPreCheckoutQueryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_answer_shipping_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerShippingQueryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_answer_web_app_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostAnswerWebAppQueryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_approve_chat_join_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostApproveChatJoinRequestError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ban_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostBanChatMemberError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ban_chat_sender_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostBanChatSenderChatError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_close`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCloseError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_close_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCloseForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_close_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCloseGeneralForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_convert_gift_to_stars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostConvertGiftToStarsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_copy_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCopyMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_copy_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCopyMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_create_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateChatInviteLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_create_chat_subscription_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateChatSubscriptionInviteLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_create_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_create_invoice_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateInvoiceLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_create_new_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreateNewStickerSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_decline_chat_join_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeclineChatJoinRequestError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_business_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteBusinessMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_chat_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteChatPhotoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_chat_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteChatStickerSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteMyCommandsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_sticker_from_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteStickerFromSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteStickerSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_story`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteStoryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostDeleteWebhookError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditChatInviteLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_chat_subscription_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditChatSubscriptionInviteLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditGeneralForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_message_caption`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageCaptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_message_live_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageLiveLocationError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_message_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageMediaError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_message_reply_markup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageReplyMarkupError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_message_text`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditMessageTextError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_story`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditStoryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_edit_user_star_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEditUserStarSubscriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_export_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostExportChatInviteLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_forward_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostForwardMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_forward_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostForwardMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_available_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetAvailableGiftsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_business_account_gifts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetBusinessAccountGiftsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_business_account_star_balance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetBusinessAccountStarBalanceError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_business_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetBusinessConnectionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_chat_administrators`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatAdministratorsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatMemberError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_chat_member_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatMemberCountError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_chat_menu_button`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetChatMenuButtonError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_custom_emoji_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetCustomEmojiStickersError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetFileError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_forum_topic_icon_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetForumTopicIconStickersError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_game_high_scores`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetGameHighScoresError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMeError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyCommandsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_my_default_administrator_rights`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyDefaultAdministratorRightsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_my_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyDescriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_my_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyNameError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_my_short_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetMyShortDescriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_star_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetStarTransactionsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetStickerSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_updates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetUpdatesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_user_chat_boosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetUserChatBoostsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_user_profile_photos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetUserProfilePhotosError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_get_webhook_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGetWebhookInfoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_gift_premium_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostGiftPremiumSubscriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_hide_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostHideGeneralForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_leave_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLeaveChatError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_log_out`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLogOutError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_pin_chat_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPinChatMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_post_story`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPostStoryError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_promote_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPromoteChatMemberError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_read_business_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReadBusinessMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_refund_star_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRefundStarPaymentError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_remove_business_account_profile_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRemoveBusinessAccountProfilePhotoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_remove_chat_verification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRemoveChatVerificationError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_remove_user_verification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRemoveUserVerificationError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_reopen_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReopenForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_reopen_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReopenGeneralForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_replace_sticker_in_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostReplaceStickerInSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_restrict_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRestrictChatMemberError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_revoke_chat_invite_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRevokeChatInviteLinkError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_save_prepared_inline_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSavePreparedInlineMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_animation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendAnimationError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_audio`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendAudioError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_chat_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendChatActionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendContactError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_dice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendDiceError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_document`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendDocumentError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendGameError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_gift`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendGiftError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_invoice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendInvoiceError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendLocationError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_media_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendMediaGroupError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_paid_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendPaidMediaError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendPhotoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_poll`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendPollError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendStickerError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_venue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVenueError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_video`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVideoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_video_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVideoNoteError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_send_voice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSendVoiceError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_business_account_bio`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountBioError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_business_account_gift_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountGiftSettingsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_business_account_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountNameError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_business_account_profile_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountProfilePhotoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_business_account_username`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetBusinessAccountUsernameError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_administrator_custom_title`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatAdministratorCustomTitleError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatDescriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_menu_button`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatMenuButtonError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatPermissionsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_photo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatPhotoError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_sticker_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatStickerSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_chat_title`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetChatTitleError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_custom_emoji_sticker_set_thumbnail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetCustomEmojiStickerSetThumbnailError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_game_score`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetGameScoreError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMessageReactionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_my_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyCommandsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_my_default_administrator_rights`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyDefaultAdministratorRightsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_my_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyDescriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_my_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyNameError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_my_short_description`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetMyShortDescriptionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_passport_data_errors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetPassportDataErrorsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_sticker_emoji_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerEmojiListError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_sticker_keywords`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerKeywordsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_sticker_mask_position`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerMaskPositionError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_sticker_position_in_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerPositionInSetError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_sticker_set_thumbnail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerSetThumbnailError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_sticker_set_title`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetStickerSetTitleError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_user_emoji_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetUserEmojiStatusError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_set_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostSetWebhookError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_stop_message_live_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostStopMessageLiveLocationError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_stop_poll`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostStopPollError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_transfer_business_account_stars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostTransferBusinessAccountStarsError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_transfer_gift`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostTransferGiftError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unban_chat_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnbanChatMemberError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unban_chat_sender_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnbanChatSenderChatError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unhide_general_forum_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnhideGeneralForumTopicError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unpin_all_chat_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinAllChatMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unpin_all_forum_topic_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinAllForumTopicMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unpin_all_general_forum_topic_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinAllGeneralForumTopicMessagesError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_unpin_chat_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUnpinChatMessageError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_upgrade_gift`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUpgradeGiftError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_upload_sticker_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUploadStickerFileError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_verify_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostVerifyChatError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_verify_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostVerifyUserError {
    Status400(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

