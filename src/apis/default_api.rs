//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-01T14:36:16.092164073Z[Etc/UTC]
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
    async fn add_sticker_to_set_post<'user_id, 'name, 'sticker>(&self, user_id: i32, name: &'name str, sticker: models::models::InputSticker) -> Result<ResponseContent<AddStickerToSetPostSuccess>, Error<AddStickerToSetPostError>>;

    /// POST /answerCallbackQuery
    ///
    /// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.  Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    async fn answer_callback_query_post<'callback_query_id, 'text, 'show_alert, 'url, 'cache_time>(&self, callback_query_id: &'callback_query_id str, text: Option<&'text str>, show_alert: Option<bool>, url: Option<&'url str>, cache_time: Option<i32>) -> Result<ResponseContent<AnswerCallbackQueryPostSuccess>, Error<AnswerCallbackQueryPostError>>;

    /// POST /answerInlineQuery
    ///
    /// Use this method to send answers to an inline query. On success, *True* is returned.   No more than **50** results per query are allowed.
    async fn answer_inline_query_post<'inline_query_id, 'results, 'cache_time, 'is_personal, 'next_offset, 'button>(&self, inline_query_id: &'inline_query_id str, results: Vec<models::InlineQueryResult>, cache_time: Option<i32>, is_personal: Option<bool>, next_offset: Option<&'next_offset str>, button: Option<models::models::InlineQueryResultsButton>) -> Result<ResponseContent<AnswerInlineQueryPostSuccess>, Error<AnswerInlineQueryPostError>>;

    /// POST /answerPreCheckoutQuery
    ///
    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\\_checkout\\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    async fn answer_pre_checkout_query_post<'pre_checkout_query_id, 'ok, 'error_message>(&self, pre_checkout_query_id: &'pre_checkout_query_id str, ok: bool, error_message: Option<&'error_message str>) -> Result<ResponseContent<AnswerPreCheckoutQueryPostSuccess>, Error<AnswerPreCheckoutQueryPostError>>;

    /// POST /answerShippingQuery
    ///
    /// If you sent an invoice requesting a shipping address and the parameter *is\\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
    async fn answer_shipping_query_post<'shipping_query_id, 'ok, 'shipping_options, 'error_message>(&self, shipping_query_id: &'shipping_query_id str, ok: bool, shipping_options: Option<Vec<models::ShippingOption>>, error_message: Option<&'error_message str>) -> Result<ResponseContent<AnswerShippingQueryPostSuccess>, Error<AnswerShippingQueryPostError>>;

    /// POST /answerWebAppQuery
    ///
    /// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
    async fn answer_web_app_query_post<'web_app_query_id, 'result>(&self, web_app_query_id: &'web_app_query_id str, result: models::models::InlineQueryResult) -> Result<ResponseContent<AnswerWebAppQueryPostSuccess>, Error<AnswerWebAppQueryPostError>>;

    /// POST /approveChatJoinRequest
    ///
    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn approve_chat_join_request_post<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, user_id: i32) -> Result<ResponseContent<ApproveChatJoinRequestPostSuccess>, Error<ApproveChatJoinRequestPostError>>;

    /// POST /banChatMember
    ///
    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn ban_chat_member_post<'chat_id, 'user_id, 'until_date, 'revoke_messages>(&self, chat_id: models::models::BanChatMemberPostRequestChatId, user_id: i32, until_date: Option<i32>, revoke_messages: Option<bool>) -> Result<ResponseContent<BanChatMemberPostSuccess>, Error<BanChatMemberPostError>>;

    /// POST /banChatSenderChat
    ///
    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn ban_chat_sender_chat_post<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<BanChatSenderChatPostSuccess>, Error<BanChatSenderChatPostError>>;

    /// POST /closeForumTopic
    ///
    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn close_forum_topic_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<CloseForumTopicPostSuccess>, Error<CloseForumTopicPostError>>;

    /// POST /closeGeneralForumTopic
    ///
    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn close_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<CloseGeneralForumTopicPostSuccess>, Error<CloseGeneralForumTopicPostError>>;

    /// POST /close
    ///
    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
    async fn close_post<>(&self, ) -> Result<ResponseContent<ClosePostSuccess>, Error<ClosePostError>>;

    /// POST /convertGiftToStars
    ///
    /// Converts a given regular gift to Telegram Stars. Requires the *can\\_convert\\_gifts\\_to\\_stars* business bot right. Returns *True* on success.
    async fn convert_gift_to_stars_post<'business_connection_id, 'owned_gift_id>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str) -> Result<ResponseContent<ConvertGiftToStarsPostSuccess>, Error<ConvertGiftToStarsPostError>>;

    /// POST /copyMessage
    ///
    /// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api/#forwardmessage), but the copied message doesn't have a link to the original message. Returns the [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent message on success.
    async fn copy_message_post<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagePostRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<CopyMessagePostSuccess>, Error<CopyMessagePostError>>;

    /// POST /copyMessages
    ///
    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn copy_messages_post<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content, 'remove_caption>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagesPostRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, remove_caption: Option<bool>) -> Result<ResponseContent<CopyMessagesPostSuccess>, Error<CopyMessagesPostError>>;

    /// POST /createChatInviteLink
    ///
    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn create_chat_invite_link_post<'chat_id, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessagePostRequestChatId, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<CreateChatInviteLinkPostSuccess>, Error<CreateChatInviteLinkPostError>>;

    /// POST /createChatSubscriptionInviteLink
    ///
    /// Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\\_invite\\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn create_chat_subscription_invite_link_post<'chat_id, 'subscription_period, 'subscription_price, 'name>(&self, chat_id: models::models::CreateChatSubscriptionInviteLinkPostRequestChatId, subscription_period: i32, subscription_price: i32, name: Option<&'name str>) -> Result<ResponseContent<CreateChatSubscriptionInviteLinkPostSuccess>, Error<CreateChatSubscriptionInviteLinkPostError>>;

    /// POST /createForumTopic
    ///
    /// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
    async fn create_forum_topic_post<'chat_id, 'name, 'icon_color, 'icon_custom_emoji_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, name: &'name str, icon_color: Option<i32>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<CreateForumTopicPostSuccess>, Error<CreateForumTopicPostError>>;

    /// POST /createInvoiceLink
    ///
    /// Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
    async fn create_invoice_link_post<'title, 'description, 'payload, 'currency, 'prices, 'business_connection_id, 'provider_token, 'subscription_period, 'max_tip_amount, 'suggested_tip_amounts, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible>(&self, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, business_connection_id: Option<&'business_connection_id str>, provider_token: Option<&'provider_token str>, subscription_period: Option<i32>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>) -> Result<ResponseContent<CreateInvoiceLinkPostSuccess>, Error<CreateInvoiceLinkPostError>>;

    /// POST /createNewStickerSet
    ///
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
    async fn create_new_sticker_set_post<'user_id, 'name, 'title, 'stickers, 'sticker_type, 'needs_repainting>(&self, user_id: i32, name: &'name str, title: &'title str, stickers: Vec<models::InputSticker>, sticker_type: Option<&'sticker_type str>, needs_repainting: Option<bool>) -> Result<ResponseContent<CreateNewStickerSetPostSuccess>, Error<CreateNewStickerSetPostError>>;

    /// POST /declineChatJoinRequest
    ///
    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn decline_chat_join_request_post<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, user_id: i32) -> Result<ResponseContent<DeclineChatJoinRequestPostSuccess>, Error<DeclineChatJoinRequestPostError>>;

    /// POST /deleteBusinessMessages
    ///
    /// Delete messages on behalf of a business account. Requires the *can\\_delete\\_sent\\_messages* business bot right to delete messages sent by the bot itself, or the *can\\_delete\\_all\\_messages* business bot right to delete any message. Returns *True* on success.
    async fn delete_business_messages_post<'business_connection_id, 'message_ids>(&self, business_connection_id: &'business_connection_id str, message_ids: Vec<i32>) -> Result<ResponseContent<DeleteBusinessMessagesPostSuccess>, Error<DeleteBusinessMessagesPostError>>;

    /// POST /deleteChatPhoto
    ///
    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn delete_chat_photo_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<DeleteChatPhotoPostSuccess>, Error<DeleteChatPhotoPostError>>;

    /// POST /deleteChatStickerSet
    ///
    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn delete_chat_sticker_set_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<DeleteChatStickerSetPostSuccess>, Error<DeleteChatStickerSetPostError>>;

    /// POST /deleteForumTopic
    ///
    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_delete\\_messages* administrator rights. Returns *True* on success.
    async fn delete_forum_topic_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<DeleteForumTopicPostSuccess>, Error<DeleteForumTopicPostError>>;

    /// POST /deleteMessage
    ///
    /// Use this method to delete a message, including service messages, with the following limitations:   \\- A message can only be deleted if it was sent less than 48 hours ago.   \\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.   \\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.   \\- Bots can delete outgoing messages in private chats, groups, and supergroups.   \\- Bots can delete incoming messages in private chats.   \\- Bots granted *can\\_post\\_messages* permissions can delete outgoing messages in channels.   \\- If the bot is an administrator of a group, it can delete any message there.   \\- If the bot has *can\\_delete\\_messages* permission in a supergroup or a channel, it can delete any message there.   Returns *True* on success.
    async fn delete_message_post<'chat_id, 'message_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32) -> Result<ResponseContent<DeleteMessagePostSuccess>, Error<DeleteMessagePostError>>;

    /// POST /deleteMessages
    ///
    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
    async fn delete_messages_post<'chat_id, 'message_ids>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_ids: Vec<i32>) -> Result<ResponseContent<DeleteMessagesPostSuccess>, Error<DeleteMessagesPostError>>;

    /// POST /deleteMyCommands
    ///
    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
    async fn delete_my_commands_post<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<DeleteMyCommandsPostSuccess>, Error<DeleteMyCommandsPostError>>;

    /// POST /deleteStickerFromSet
    ///
    /// Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
    async fn delete_sticker_from_set_post<'sticker>(&self, sticker: &'sticker str) -> Result<ResponseContent<DeleteStickerFromSetPostSuccess>, Error<DeleteStickerFromSetPostError>>;

    /// POST /deleteStickerSet
    ///
    /// Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
    async fn delete_sticker_set_post<'name>(&self, name: &'name str) -> Result<ResponseContent<DeleteStickerSetPostSuccess>, Error<DeleteStickerSetPostError>>;

    /// POST /deleteStory
    ///
    /// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns *True* on success.
    async fn delete_story_post<'business_connection_id, 'story_id>(&self, business_connection_id: &'business_connection_id str, story_id: i32) -> Result<ResponseContent<DeleteStoryPostSuccess>, Error<DeleteStoryPostError>>;

    /// POST /deleteWebhook
    ///
    /// Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
    async fn delete_webhook_post<'drop_pending_updates>(&self, drop_pending_updates: Option<bool>) -> Result<ResponseContent<DeleteWebhookPostSuccess>, Error<DeleteWebhookPostError>>;

    /// POST /editChatInviteLink
    ///
    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn edit_chat_invite_link_post<'chat_id, 'invite_link, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessagePostRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<EditChatInviteLinkPostSuccess>, Error<EditChatInviteLinkPostError>>;

    /// POST /editChatSubscriptionInviteLink
    ///
    /// Use this method to edit a subscription invite link created by the bot. The bot must have the *can\\_invite\\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn edit_chat_subscription_invite_link_post<'chat_id, 'invite_link, 'name>(&self, chat_id: models::models::SendMessagePostRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>) -> Result<ResponseContent<EditChatSubscriptionInviteLinkPostSuccess>, Error<EditChatSubscriptionInviteLinkPostError>>;

    /// POST /editForumTopic
    ///
    /// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn edit_forum_topic_post<'chat_id, 'message_thread_id, 'name, 'icon_custom_emoji_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32, name: Option<&'name str>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<EditForumTopicPostSuccess>, Error<EditForumTopicPostError>>;

    /// POST /editGeneralForumTopic
    ///
    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn edit_general_forum_topic_post<'chat_id, 'name>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, name: &'name str) -> Result<ResponseContent<EditGeneralForumTopicPostSuccess>, Error<EditGeneralForumTopicPostError>>;

    /// POST /editMessageCaption
    ///
    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_caption_post<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageCaptionPostSuccess>, Error<EditMessageCaptionPostError>>;

    /// POST /editMessageLiveLocation
    ///
    /// Use this method to edit live location messages. A location can be edited until its *live\\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn edit_message_live_location_post<'latitude, 'longitude, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'live_period, 'horizontal_accuracy, 'heading, 'proximity_alert_radius, 'reply_markup>(&self, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, live_period: Option<i32>, horizontal_accuracy: Option<f64>, heading: Option<i32>, proximity_alert_radius: Option<i32>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageLiveLocationPostSuccess>, Error<EditMessageLiveLocationPostError>>;

    /// POST /editMessageMedia
    ///
    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_media_post<'media, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, media: models::models::InputMedia, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageMediaPostSuccess>, Error<EditMessageMediaPostError>>;

    /// POST /editMessageReplyMarkup
    ///
    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_reply_markup_post<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageReplyMarkupPostSuccess>, Error<EditMessageReplyMarkupPostError>>;

    /// POST /editMessageText
    ///
    /// Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_text_post<'text, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'parse_mode, 'entities, 'link_preview_options, 'reply_markup>(&self, text: &'text str, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageTextPostSuccess>, Error<EditMessageTextPostError>>;

    /// POST /editStory
    ///
    /// Edits a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn edit_story_post<'business_connection_id, 'story_id, 'content, 'caption, 'parse_mode, 'caption_entities, 'areas>(&self, business_connection_id: &'business_connection_id str, story_id: i32, content: models::models::InputStoryContent, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>) -> Result<ResponseContent<EditStoryPostSuccess>, Error<EditStoryPostError>>;

    /// POST /editUserStarSubscription
    ///
    /// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
    async fn edit_user_star_subscription_post<'user_id, 'telegram_payment_charge_id, 'is_canceled>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str, is_canceled: bool) -> Result<ResponseContent<EditUserStarSubscriptionPostSuccess>, Error<EditUserStarSubscriptionPostError>>;

    /// POST /exportChatInviteLink
    ///
    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
    async fn export_chat_invite_link_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<ExportChatInviteLinkPostSuccess>, Error<ExportChatInviteLinkPostError>>;

    /// POST /forwardMessage
    ///
    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn forward_message_post<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagePostRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<ForwardMessagePostSuccess>, Error<ForwardMessagePostError>>;

    /// POST /forwardMessages
    ///
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn forward_messages_post<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagesPostRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<ForwardMessagesPostSuccess>, Error<ForwardMessagesPostError>>;

    /// POST /getAvailableGifts
    ///
    /// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
    async fn get_available_gifts_post<>(&self, ) -> Result<ResponseContent<GetAvailableGiftsPostSuccess>, Error<GetAvailableGiftsPostError>>;

    /// POST /getBusinessAccountGifts
    ///
    /// Returns the gifts received and owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [OwnedGifts](https://core.telegram.org/bots/api/#ownedgifts) on success.
    async fn get_business_account_gifts_post<'business_connection_id, 'exclude_unsaved, 'exclude_saved, 'exclude_unlimited, 'exclude_limited, 'exclude_unique, 'sort_by_price, 'offset, 'limit>(&self, business_connection_id: &'business_connection_id str, exclude_unsaved: Option<bool>, exclude_saved: Option<bool>, exclude_unlimited: Option<bool>, exclude_limited: Option<bool>, exclude_unique: Option<bool>, sort_by_price: Option<bool>, offset: Option<&'offset str>, limit: Option<i32>) -> Result<ResponseContent<GetBusinessAccountGiftsPostSuccess>, Error<GetBusinessAccountGiftsPostError>>;

    /// POST /getBusinessAccountStarBalance
    ///
    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [StarAmount](https://core.telegram.org/bots/api/#staramount) on success.
    async fn get_business_account_star_balance_post<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<GetBusinessAccountStarBalancePostSuccess>, Error<GetBusinessAccountStarBalancePostError>>;

    /// POST /getBusinessConnection
    ///
    /// Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
    async fn get_business_connection_post<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<GetBusinessConnectionPostSuccess>, Error<GetBusinessConnectionPostError>>;

    /// POST /getChatAdministrators
    ///
    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
    async fn get_chat_administrators_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<GetChatAdministratorsPostSuccess>, Error<GetChatAdministratorsPostError>>;

    /// POST /getChatMemberCount
    ///
    /// Use this method to get the number of members in a chat. Returns *Int* on success.
    async fn get_chat_member_count_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<GetChatMemberCountPostSuccess>, Error<GetChatMemberCountPostError>>;

    /// POST /getChatMember
    ///
    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
    async fn get_chat_member_post<'chat_id, 'user_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId, user_id: i32) -> Result<ResponseContent<GetChatMemberPostSuccess>, Error<GetChatMemberPostError>>;

    /// POST /getChatMenuButton
    ///
    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
    async fn get_chat_menu_button_post<'chat_id>(&self, chat_id: Option<i32>) -> Result<ResponseContent<GetChatMenuButtonPostSuccess>, Error<GetChatMenuButtonPostError>>;

    /// POST /getChat
    ///
    /// Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
    async fn get_chat_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<GetChatPostSuccess>, Error<GetChatPostError>>;

    /// POST /getCustomEmojiStickers
    ///
    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn get_custom_emoji_stickers_post<'custom_emoji_ids>(&self, custom_emoji_ids: Vec<String>) -> Result<ResponseContent<GetCustomEmojiStickersPostSuccess>, Error<GetCustomEmojiStickersPostError>>;

    /// POST /getFile
    ///
    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
    async fn get_file_post<'file_id>(&self, file_id: &'file_id str) -> Result<ResponseContent<GetFilePostSuccess>, Error<GetFilePostError>>;

    /// POST /getForumTopicIconStickers
    ///
    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn get_forum_topic_icon_stickers_post<>(&self, ) -> Result<ResponseContent<GetForumTopicIconStickersPostSuccess>, Error<GetForumTopicIconStickersPostError>>;

    /// POST /getGameHighScores
    ///
    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.  This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
    async fn get_game_high_scores_post<'user_id, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<GetGameHighScoresPostSuccess>, Error<GetGameHighScoresPostError>>;

    /// POST /getMe
    ///
    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
    async fn get_me_post<>(&self, ) -> Result<ResponseContent<GetMePostSuccess>, Error<GetMePostError>>;

    /// POST /getMyCommands
    ///
    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
    async fn get_my_commands_post<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyCommandsPostSuccess>, Error<GetMyCommandsPostError>>;

    /// POST /getMyDefaultAdministratorRights
    ///
    /// Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
    async fn get_my_default_administrator_rights_post<'for_channels>(&self, for_channels: Option<bool>) -> Result<ResponseContent<GetMyDefaultAdministratorRightsPostSuccess>, Error<GetMyDefaultAdministratorRightsPostError>>;

    /// POST /getMyDescription
    ///
    /// Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
    async fn get_my_description_post<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyDescriptionPostSuccess>, Error<GetMyDescriptionPostError>>;

    /// POST /getMyName
    ///
    /// Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
    async fn get_my_name_post<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyNamePostSuccess>, Error<GetMyNamePostError>>;

    /// POST /getMyShortDescription
    ///
    /// Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
    async fn get_my_short_description_post<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyShortDescriptionPostSuccess>, Error<GetMyShortDescriptionPostError>>;

    /// POST /getStarTransactions
    ///
    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
    async fn get_star_transactions_post<'offset, 'limit>(&self, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<GetStarTransactionsPostSuccess>, Error<GetStarTransactionsPostError>>;

    /// POST /getStickerSet
    ///
    /// Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
    async fn get_sticker_set_post<'name>(&self, name: &'name str) -> Result<ResponseContent<GetStickerSetPostSuccess>, Error<GetStickerSetPostError>>;

    /// POST /getUpdates
    ///
    /// Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.
    async fn get_updates_post<'offset, 'limit, 'timeout, 'allowed_updates>(&self, offset: Option<i32>, limit: Option<i32>, timeout: Option<i32>, allowed_updates: Option<Vec<String>>) -> Result<ResponseContent<GetUpdatesPostSuccess>, Error<GetUpdatesPostError>>;

    /// POST /getUserChatBoosts
    ///
    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
    async fn get_user_chat_boosts_post<'chat_id, 'user_id>(&self, chat_id: models::models::GetUserChatBoostsPostRequestChatId, user_id: i32) -> Result<ResponseContent<GetUserChatBoostsPostSuccess>, Error<GetUserChatBoostsPostError>>;

    /// POST /getUserProfilePhotos
    ///
    /// Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
    async fn get_user_profile_photos_post<'user_id, 'offset, 'limit>(&self, user_id: i32, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<GetUserProfilePhotosPostSuccess>, Error<GetUserProfilePhotosPostError>>;

    /// POST /getWebhookInfo
    ///
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
    async fn get_webhook_info_post<>(&self, ) -> Result<ResponseContent<GetWebhookInfoPostSuccess>, Error<GetWebhookInfoPostError>>;

    /// POST /giftPremiumSubscription
    ///
    /// Gifts a Telegram Premium subscription to the given user. Returns *True* on success.
    async fn gift_premium_subscription_post<'user_id, 'month_count, 'star_count, 'text, 'text_parse_mode, 'text_entities>(&self, user_id: i32, month_count: i32, star_count: i32, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<GiftPremiumSubscriptionPostSuccess>, Error<GiftPremiumSubscriptionPostError>>;

    /// POST /hideGeneralForumTopic
    ///
    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
    async fn hide_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<HideGeneralForumTopicPostSuccess>, Error<HideGeneralForumTopicPostError>>;

    /// POST /leaveChat
    ///
    /// Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    async fn leave_chat_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<LeaveChatPostSuccess>, Error<LeaveChatPostError>>;

    /// POST /logOut
    ///
    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
    async fn log_out_post<>(&self, ) -> Result<ResponseContent<LogOutPostSuccess>, Error<LogOutPostError>>;

    /// POST /pinChatMessage
    ///
    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn pin_chat_message_post<'chat_id, 'message_id, 'business_connection_id, 'disable_notification>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, disable_notification: Option<bool>) -> Result<ResponseContent<PinChatMessagePostSuccess>, Error<PinChatMessagePostError>>;

    /// POST /postStory
    ///
    /// Posts a story on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn post_story_post<'business_connection_id, 'content, 'active_period, 'caption, 'parse_mode, 'caption_entities, 'areas, 'post_to_chat_page, 'protect_content>(&self, business_connection_id: &'business_connection_id str, content: models::models::InputStoryContent, active_period: i32, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>, post_to_chat_page: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostStoryPostSuccess>, Error<PostStoryPostError>>;

    /// POST /promoteChatMember
    ///
    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    async fn promote_chat_member_post<'chat_id, 'user_id, 'is_anonymous, 'can_manage_chat, 'can_delete_messages, 'can_manage_video_chats, 'can_restrict_members, 'can_promote_members, 'can_change_info, 'can_invite_users, 'can_post_stories, 'can_edit_stories, 'can_delete_stories, 'can_post_messages, 'can_edit_messages, 'can_pin_messages, 'can_manage_topics>(&self, chat_id: models::models::SendMessagePostRequestChatId, user_id: i32, is_anonymous: Option<bool>, can_manage_chat: Option<bool>, can_delete_messages: Option<bool>, can_manage_video_chats: Option<bool>, can_restrict_members: Option<bool>, can_promote_members: Option<bool>, can_change_info: Option<bool>, can_invite_users: Option<bool>, can_post_stories: Option<bool>, can_edit_stories: Option<bool>, can_delete_stories: Option<bool>, can_post_messages: Option<bool>, can_edit_messages: Option<bool>, can_pin_messages: Option<bool>, can_manage_topics: Option<bool>) -> Result<ResponseContent<PromoteChatMemberPostSuccess>, Error<PromoteChatMemberPostError>>;

    /// POST /readBusinessMessage
    ///
    /// Marks incoming message as read on behalf of a business account. Requires the *can\\_read\\_messages* business bot right. Returns *True* on success.
    async fn read_business_message_post<'business_connection_id, 'chat_id, 'message_id>(&self, business_connection_id: &'business_connection_id str, chat_id: i32, message_id: i32) -> Result<ResponseContent<ReadBusinessMessagePostSuccess>, Error<ReadBusinessMessagePostError>>;

    /// POST /refundStarPayment
    ///
    /// Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
    async fn refund_star_payment_post<'user_id, 'telegram_payment_charge_id>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str) -> Result<ResponseContent<RefundStarPaymentPostSuccess>, Error<RefundStarPaymentPostError>>;

    /// POST /removeBusinessAccountProfilePhoto
    ///
    /// Removes the current profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn remove_business_account_profile_photo_post<'business_connection_id, 'is_public>(&self, business_connection_id: &'business_connection_id str, is_public: Option<bool>) -> Result<ResponseContent<RemoveBusinessAccountProfilePhotoPostSuccess>, Error<RemoveBusinessAccountProfilePhotoPostError>>;

    /// POST /removeChatVerification
    ///
    /// Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn remove_chat_verification_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<RemoveChatVerificationPostSuccess>, Error<RemoveChatVerificationPostError>>;

    /// POST /removeUserVerification
    ///
    /// Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn remove_user_verification_post<'user_id>(&self, user_id: i32) -> Result<ResponseContent<RemoveUserVerificationPostSuccess>, Error<RemoveUserVerificationPostError>>;

    /// POST /reopenForumTopic
    ///
    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn reopen_forum_topic_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<ReopenForumTopicPostSuccess>, Error<ReopenForumTopicPostError>>;

    /// POST /reopenGeneralForumTopic
    ///
    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
    async fn reopen_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<ReopenGeneralForumTopicPostSuccess>, Error<ReopenGeneralForumTopicPostError>>;

    /// POST /replaceStickerInSet
    ///
    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
    async fn replace_sticker_in_set_post<'user_id, 'name, 'old_sticker, 'sticker>(&self, user_id: i32, name: &'name str, old_sticker: &'old_sticker str, sticker: models::models::InputSticker) -> Result<ResponseContent<ReplaceStickerInSetPostSuccess>, Error<ReplaceStickerInSetPostError>>;

    /// POST /restrictChatMember
    ///
    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
    async fn restrict_chat_member_post<'chat_id, 'user_id, 'permissions, 'use_independent_chat_permissions, 'until_date>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, user_id: i32, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>, until_date: Option<i32>) -> Result<ResponseContent<RestrictChatMemberPostSuccess>, Error<RestrictChatMemberPostError>>;

    /// POST /revokeChatInviteLink
    ///
    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn revoke_chat_invite_link_post<'chat_id, 'invite_link>(&self, chat_id: models::models::RevokeChatInviteLinkPostRequestChatId, invite_link: &'invite_link str) -> Result<ResponseContent<RevokeChatInviteLinkPostSuccess>, Error<RevokeChatInviteLinkPostError>>;

    /// POST /savePreparedInlineMessage
    ///
    /// Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
    async fn save_prepared_inline_message_post<'user_id, 'result, 'allow_user_chats, 'allow_bot_chats, 'allow_group_chats, 'allow_channel_chats>(&self, user_id: i32, result: models::models::InlineQueryResult, allow_user_chats: Option<bool>, allow_bot_chats: Option<bool>, allow_group_chats: Option<bool>, allow_channel_chats: Option<bool>) -> Result<ResponseContent<SavePreparedInlineMessagePostSuccess>, Error<SavePreparedInlineMessagePostError>>;

    /// POST /sendAnimation
    ///
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    async fn send_animation_post<'chat_id, 'animation, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, animation: models::models::SendAnimationPostRequestAnimation, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendAnimationPostSuccess>, Error<SendAnimationPostError>>;

    /// POST /sendAudio
    ///
    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.  For sending voice messages, use the [sendVoice](https://core.telegram.org/bots/api/#sendvoice) method instead.
    async fn send_audio_post<'chat_id, 'audio, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'performer, 'title, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, audio: models::models::SendAudioPostRequestAudio, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, performer: Option<&'performer str>, title: Option<&'title str>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendAudioPostSuccess>, Error<SendAudioPostError>>;

    /// POST /sendChatAction
    ///
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.  Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\\_photo*. The user will see a “sending photo” status for the bot.  We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    async fn send_chat_action_post<'chat_id, 'action, 'business_connection_id, 'message_thread_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, action: &'action str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>) -> Result<ResponseContent<SendChatActionPostSuccess>, Error<SendChatActionPostError>>;

    /// POST /sendContact
    ///
    /// Use this method to send phone contacts. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_contact_post<'chat_id, 'phone_number, 'first_name, 'business_connection_id, 'message_thread_id, 'last_name, 'vcard, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, phone_number: &'phone_number str, first_name: &'first_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, last_name: Option<&'last_name str>, vcard: Option<&'vcard str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendContactPostSuccess>, Error<SendContactPostError>>;

    /// POST /sendDice
    ///
    /// Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_dice_post<'chat_id, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendDicePostSuccess>, Error<SendDicePostError>>;

    /// POST /sendDocument
    ///
    /// Use this method to send general files. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    async fn send_document_post<'chat_id, 'document, 'business_connection_id, 'message_thread_id, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'disable_content_type_detection, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, document: models::models::SendDocumentPostRequestDocument, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, disable_content_type_detection: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendDocumentPostSuccess>, Error<SendDocumentPostError>>;

    /// POST /sendGame
    ///
    /// Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_game_post<'chat_id, 'game_short_name, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: i32, game_short_name: &'game_short_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<SendGamePostSuccess>, Error<SendGamePostError>>;

    /// POST /sendGift
    ///
    /// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.
    async fn send_gift_post<'gift_id, 'user_id, 'chat_id, 'pay_for_upgrade, 'text, 'text_parse_mode, 'text_entities>(&self, gift_id: &'gift_id str, user_id: Option<i32>, chat_id: Option<models::models::SendGiftPostRequestChatId>, pay_for_upgrade: Option<bool>, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<SendGiftPostSuccess>, Error<SendGiftPostError>>;

    /// POST /sendInvoice
    ///
    /// Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_invoice_post<'chat_id, 'title, 'description, 'payload, 'currency, 'prices, 'message_thread_id, 'provider_token, 'max_tip_amount, 'suggested_tip_amounts, 'start_parameter, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, message_thread_id: Option<i32>, provider_token: Option<&'provider_token str>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, start_parameter: Option<&'start_parameter str>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<SendInvoicePostSuccess>, Error<SendInvoicePostError>>;

    /// POST /sendLocation
    ///
    /// Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_location_post<'chat_id, 'latitude, 'longitude, 'business_connection_id, 'message_thread_id, 'horizontal_accuracy, 'live_period, 'heading, 'proximity_alert_radius, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, horizontal_accuracy: Option<f64>, live_period: Option<i32>, heading: Option<i32>, proximity_alert_radius: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendLocationPostSuccess>, Error<SendLocationPostError>>;

    /// POST /sendMediaGroup
    ///
    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
    async fn send_media_group_post<'chat_id, 'media, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters>(&self, chat_id: models::models::SendMessagePostRequestChatId, media: Vec<models::SendMediaGroupPostRequestMediaInner>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>) -> Result<ResponseContent<SendMediaGroupPostSuccess>, Error<SendMediaGroupPostError>>;

    /// POST /sendMessage
    ///
    /// Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_message_post<'chat_id, 'text, 'business_connection_id, 'message_thread_id, 'parse_mode, 'entities, 'link_preview_options, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, text: &'text str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendMessagePostSuccess>, Error<SendMessagePostError>>;

    /// POST /sendPaidMedia
    ///
    /// Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_paid_media_post<'chat_id, 'star_count, 'media, 'business_connection_id, 'payload, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendPaidMediaPostRequestChatId, star_count: i32, media: Vec<models::InputPaidMedia>, business_connection_id: Option<&'business_connection_id str>, payload: Option<&'payload str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendPaidMediaPostSuccess>, Error<SendPaidMediaPostError>>;

    /// POST /sendPhoto
    ///
    /// Use this method to send photos. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_photo_post<'chat_id, 'photo, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, photo: models::models::SendPhotoPostRequestPhoto, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendPhotoPostSuccess>, Error<SendPhotoPostError>>;

    /// POST /sendPoll
    ///
    /// Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_poll_post<'chat_id, 'question, 'options, 'business_connection_id, 'message_thread_id, 'question_parse_mode, 'question_entities, 'is_anonymous, 'r_type, 'allows_multiple_answers, 'correct_option_id, 'explanation, 'explanation_parse_mode, 'explanation_entities, 'open_period, 'close_date, 'is_closed, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, question: &'question str, options: Vec<models::InputPollOption>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, question_parse_mode: Option<&'question_parse_mode str>, question_entities: Option<Vec<models::MessageEntity>>, is_anonymous: Option<bool>, r#type: Option<&'r_type str>, allows_multiple_answers: Option<bool>, correct_option_id: Option<i32>, explanation: Option<&'explanation str>, explanation_parse_mode: Option<&'explanation_parse_mode str>, explanation_entities: Option<Vec<models::MessageEntity>>, open_period: Option<i32>, close_date: Option<i32>, is_closed: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendPollPostSuccess>, Error<SendPollPostError>>;

    /// POST /sendSticker
    ///
    /// Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_sticker_post<'chat_id, 'sticker, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, sticker: models::models::SendStickerPostRequestSticker, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendStickerPostSuccess>, Error<SendStickerPostError>>;

    /// POST /sendVenue
    ///
    /// Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_venue_post<'chat_id, 'latitude, 'longitude, 'title, 'address, 'business_connection_id, 'message_thread_id, 'foursquare_id, 'foursquare_type, 'google_place_id, 'google_place_type, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, latitude: f64, longitude: f64, title: &'title str, address: &'address str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, foursquare_id: Option<&'foursquare_id str>, foursquare_type: Option<&'foursquare_type str>, google_place_id: Option<&'google_place_id str>, google_place_type: Option<&'google_place_type str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVenuePostSuccess>, Error<SendVenuePostError>>;

    /// POST /sendVideoNote
    ///
    /// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_video_note_post<'chat_id, 'video_note, 'business_connection_id, 'message_thread_id, 'duration, 'length, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, video_note: models::models::SendVideoNotePostRequestVideoNote, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, length: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVideoNotePostSuccess>, Error<SendVideoNotePostError>>;

    /// POST /sendVideo
    ///
    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    async fn send_video_post<'chat_id, 'video, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'cover, 'start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'supports_streaming, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, video: models::models::SendVideoPostRequestVideo, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, cover: Option<models::models::SendVideoPostRequestCover>, start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, supports_streaming: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVideoPostSuccess>, Error<SendVideoPostError>>;

    /// POST /sendVoice
    ///
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as [Audio](https://core.telegram.org/bots/api/#audio) or [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    async fn send_voice_post<'chat_id, 'voice, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, voice: models::models::SendVoicePostRequestVoice, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVoicePostSuccess>, Error<SendVoicePostError>>;

    /// POST /setBusinessAccountBio
    ///
    /// Changes the bio of a managed business account. Requires the *can\\_change\\_bio* business bot right. Returns *True* on success.
    async fn set_business_account_bio_post<'business_connection_id, 'bio>(&self, business_connection_id: &'business_connection_id str, bio: Option<&'bio str>) -> Result<ResponseContent<SetBusinessAccountBioPostSuccess>, Error<SetBusinessAccountBioPostError>>;

    /// POST /setBusinessAccountGiftSettings
    ///
    /// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the *can\\_change\\_gift\\_settings* business bot right. Returns *True* on success.
    async fn set_business_account_gift_settings_post<'business_connection_id, 'show_gift_button, 'accepted_gift_types>(&self, business_connection_id: &'business_connection_id str, show_gift_button: bool, accepted_gift_types: models::models::AcceptedGiftTypes) -> Result<ResponseContent<SetBusinessAccountGiftSettingsPostSuccess>, Error<SetBusinessAccountGiftSettingsPostError>>;

    /// POST /setBusinessAccountName
    ///
    /// Changes the first and last name of a managed business account. Requires the *can\\_change\\_name* business bot right. Returns *True* on success.
    async fn set_business_account_name_post<'business_connection_id, 'first_name, 'last_name>(&self, business_connection_id: &'business_connection_id str, first_name: &'first_name str, last_name: Option<&'last_name str>) -> Result<ResponseContent<SetBusinessAccountNamePostSuccess>, Error<SetBusinessAccountNamePostError>>;

    /// POST /setBusinessAccountProfilePhoto
    ///
    /// Changes the profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn set_business_account_profile_photo_post<'business_connection_id, 'photo, 'is_public>(&self, business_connection_id: &'business_connection_id str, photo: models::models::InputProfilePhoto, is_public: Option<bool>) -> Result<ResponseContent<SetBusinessAccountProfilePhotoPostSuccess>, Error<SetBusinessAccountProfilePhotoPostError>>;

    /// POST /setBusinessAccountUsername
    ///
    /// Changes the username of a managed business account. Requires the *can\\_change\\_username* business bot right. Returns *True* on success.
    async fn set_business_account_username_post<'business_connection_id, 'username>(&self, business_connection_id: &'business_connection_id str, username: Option<&'username str>) -> Result<ResponseContent<SetBusinessAccountUsernamePostSuccess>, Error<SetBusinessAccountUsernamePostError>>;

    /// POST /setChatAdministratorCustomTitle
    ///
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
    async fn set_chat_administrator_custom_title_post<'chat_id, 'user_id, 'custom_title>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, user_id: i32, custom_title: &'custom_title str) -> Result<ResponseContent<SetChatAdministratorCustomTitlePostSuccess>, Error<SetChatAdministratorCustomTitlePostError>>;

    /// POST /setChatDescription
    ///
    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn set_chat_description_post<'chat_id, 'description>(&self, chat_id: models::models::SendMessagePostRequestChatId, description: Option<&'description str>) -> Result<ResponseContent<SetChatDescriptionPostSuccess>, Error<SetChatDescriptionPostError>>;

    /// POST /setChatMenuButton
    ///
    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
    async fn set_chat_menu_button_post<'chat_id, 'menu_button>(&self, chat_id: Option<i32>, menu_button: Option<models::models::MenuButton>) -> Result<ResponseContent<SetChatMenuButtonPostSuccess>, Error<SetChatMenuButtonPostError>>;

    /// POST /setChatPermissions
    ///
    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\\_restrict\\_members* administrator rights. Returns *True* on success.
    async fn set_chat_permissions_post<'chat_id, 'permissions, 'use_independent_chat_permissions>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>) -> Result<ResponseContent<SetChatPermissionsPostSuccess>, Error<SetChatPermissionsPostError>>;

    /// POST /setChatPhoto
    ///
    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn set_chat_photo_post<'chat_id, 'photo>(&self, chat_id: models::models::SendMessagePostRequestChatId, photo: Option<models::serde_json::Value>) -> Result<ResponseContent<SetChatPhotoPostSuccess>, Error<SetChatPhotoPostError>>;

    /// POST /setChatStickerSet
    ///
    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn set_chat_sticker_set_post<'chat_id, 'sticker_set_name>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, sticker_set_name: &'sticker_set_name str) -> Result<ResponseContent<SetChatStickerSetPostSuccess>, Error<SetChatStickerSetPostError>>;

    /// POST /setChatTitle
    ///
    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn set_chat_title_post<'chat_id, 'title>(&self, chat_id: models::models::SendMessagePostRequestChatId, title: &'title str) -> Result<ResponseContent<SetChatTitlePostSuccess>, Error<SetChatTitlePostError>>;

    /// POST /setCustomEmojiStickerSetThumbnail
    ///
    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
    async fn set_custom_emoji_sticker_set_thumbnail_post<'name, 'custom_emoji_id>(&self, name: &'name str, custom_emoji_id: Option<&'custom_emoji_id str>) -> Result<ResponseContent<SetCustomEmojiStickerSetThumbnailPostSuccess>, Error<SetCustomEmojiStickerSetThumbnailPostError>>;

    /// POST /setGameScore
    ///
    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
    async fn set_game_score_post<'user_id, 'score, 'force, 'disable_edit_message, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, score: i32, force: Option<bool>, disable_edit_message: Option<bool>, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<SetGameScorePostSuccess>, Error<SetGameScorePostError>>;

    /// POST /setMessageReaction
    ///
    /// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.
    async fn set_message_reaction_post<'chat_id, 'message_id, 'reaction, 'is_big>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32, reaction: Option<Vec<models::ReactionType>>, is_big: Option<bool>) -> Result<ResponseContent<SetMessageReactionPostSuccess>, Error<SetMessageReactionPostError>>;

    /// POST /setMyCommands
    ///
    /// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
    async fn set_my_commands_post<'commands, 'scope, 'language_code>(&self, commands: Vec<models::BotCommand>, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyCommandsPostSuccess>, Error<SetMyCommandsPostError>>;

    /// POST /setMyDefaultAdministratorRights
    ///
    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
    async fn set_my_default_administrator_rights_post<'rights, 'for_channels>(&self, rights: Option<models::models::ChatAdministratorRights>, for_channels: Option<bool>) -> Result<ResponseContent<SetMyDefaultAdministratorRightsPostSuccess>, Error<SetMyDefaultAdministratorRightsPostError>>;

    /// POST /setMyDescription
    ///
    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
    async fn set_my_description_post<'description, 'language_code>(&self, description: Option<&'description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyDescriptionPostSuccess>, Error<SetMyDescriptionPostError>>;

    /// POST /setMyName
    ///
    /// Use this method to change the bot's name. Returns *True* on success.
    async fn set_my_name_post<'name, 'language_code>(&self, name: Option<&'name str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyNamePostSuccess>, Error<SetMyNamePostError>>;

    /// POST /setMyShortDescription
    ///
    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
    async fn set_my_short_description_post<'short_description, 'language_code>(&self, short_description: Option<&'short_description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyShortDescriptionPostSuccess>, Error<SetMyShortDescriptionPostError>>;

    /// POST /setPassportDataErrors
    ///
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.  Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    async fn set_passport_data_errors_post<'user_id, 'errors>(&self, user_id: i32, errors: Vec<models::PassportElementError>) -> Result<ResponseContent<SetPassportDataErrorsPostSuccess>, Error<SetPassportDataErrorsPostError>>;

    /// POST /setStickerEmojiList
    ///
    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn set_sticker_emoji_list_post<'sticker, 'emoji_list>(&self, sticker: &'sticker str, emoji_list: Vec<String>) -> Result<ResponseContent<SetStickerEmojiListPostSuccess>, Error<SetStickerEmojiListPostError>>;

    /// POST /setStickerKeywords
    ///
    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn set_sticker_keywords_post<'sticker, 'keywords>(&self, sticker: &'sticker str, keywords: Option<Vec<String>>) -> Result<ResponseContent<SetStickerKeywordsPostSuccess>, Error<SetStickerKeywordsPostError>>;

    /// POST /setStickerMaskPosition
    ///
    /// Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
    async fn set_sticker_mask_position_post<'sticker, 'mask_position>(&self, sticker: &'sticker str, mask_position: Option<models::models::MaskPosition>) -> Result<ResponseContent<SetStickerMaskPositionPostSuccess>, Error<SetStickerMaskPositionPostError>>;

    /// POST /setStickerPositionInSet
    ///
    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
    async fn set_sticker_position_in_set_post<'sticker, 'position>(&self, sticker: &'sticker str, position: i32) -> Result<ResponseContent<SetStickerPositionInSetPostSuccess>, Error<SetStickerPositionInSetPostError>>;

    /// POST /setStickerSetThumbnail
    ///
    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
    async fn set_sticker_set_thumbnail_post<'name, 'user_id, 'format, 'thumbnail>(&self, name: &'name str, user_id: i32, format: &'format str, thumbnail: Option<models::models::SetStickerSetThumbnailPostRequestThumbnail>) -> Result<ResponseContent<SetStickerSetThumbnailPostSuccess>, Error<SetStickerSetThumbnailPostError>>;

    /// POST /setStickerSetTitle
    ///
    /// Use this method to set the title of a created sticker set. Returns *True* on success.
    async fn set_sticker_set_title_post<'name, 'title>(&self, name: &'name str, title: &'title str) -> Result<ResponseContent<SetStickerSetTitlePostSuccess>, Error<SetStickerSetTitlePostError>>;

    /// POST /setUserEmojiStatus
    ///
    /// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
    async fn set_user_emoji_status_post<'user_id, 'emoji_status_custom_emoji_id, 'emoji_status_expiration_date>(&self, user_id: i32, emoji_status_custom_emoji_id: Option<&'emoji_status_custom_emoji_id str>, emoji_status_expiration_date: Option<i32>) -> Result<ResponseContent<SetUserEmojiStatusPostSuccess>, Error<SetUserEmojiStatusPostError>>;

    /// POST /setWebhook
    ///
    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts. Returns *True* on success.  If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    async fn set_webhook_post<'url, 'certificate, 'ip_address, 'max_connections, 'allowed_updates, 'drop_pending_updates, 'secret_token>(&self, url: &'url str, certificate: Option<models::serde_json::Value>, ip_address: Option<&'ip_address str>, max_connections: Option<i32>, allowed_updates: Option<Vec<String>>, drop_pending_updates: Option<bool>, secret_token: Option<&'secret_token str>) -> Result<ResponseContent<SetWebhookPostSuccess>, Error<SetWebhookPostError>>;

    /// POST /stopMessageLiveLocation
    ///
    /// Use this method to stop updating a live location message before *live\\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn stop_message_live_location_post<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<StopMessageLiveLocationPostSuccess>, Error<StopMessageLiveLocationPostError>>;

    /// POST /stopPoll
    ///
    /// Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
    async fn stop_poll_post<'chat_id, 'message_id, 'business_connection_id, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<StopPollPostSuccess>, Error<StopPollPostError>>;

    /// POST /transferBusinessAccountStars
    ///
    /// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the *can\\_transfer\\_stars* business bot right. Returns *True* on success.
    async fn transfer_business_account_stars_post<'business_connection_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, star_count: i32) -> Result<ResponseContent<TransferBusinessAccountStarsPostSuccess>, Error<TransferBusinessAccountStarsPostError>>;

    /// POST /transferGift
    ///
    /// Transfers an owned unique gift to another user. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Requires *can\\_transfer\\_stars* business bot right if the transfer is paid. Returns *True* on success.
    async fn transfer_gift_post<'business_connection_id, 'owned_gift_id, 'new_owner_chat_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, new_owner_chat_id: i32, star_count: Option<i32>) -> Result<ResponseContent<TransferGiftPostSuccess>, Error<TransferGiftPostError>>;

    /// POST /unbanChatMember
    ///
    /// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\\_if\\_banned*. Returns *True* on success.
    async fn unban_chat_member_post<'chat_id, 'user_id, 'only_if_banned>(&self, chat_id: models::models::BanChatMemberPostRequestChatId, user_id: i32, only_if_banned: Option<bool>) -> Result<ResponseContent<UnbanChatMemberPostSuccess>, Error<UnbanChatMemberPostError>>;

    /// POST /unbanChatSenderChat
    ///
    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn unban_chat_sender_chat_post<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<UnbanChatSenderChatPostSuccess>, Error<UnbanChatSenderChatPostError>>;

    /// POST /unhideGeneralForumTopic
    ///
    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn unhide_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<UnhideGeneralForumTopicPostSuccess>, Error<UnhideGeneralForumTopicPostError>>;

    /// POST /unpinAllChatMessages
    ///
    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn unpin_all_chat_messages_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<UnpinAllChatMessagesPostSuccess>, Error<UnpinAllChatMessagesPostError>>;

    /// POST /unpinAllForumTopicMessages
    ///
    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn unpin_all_forum_topic_messages_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<UnpinAllForumTopicMessagesPostSuccess>, Error<UnpinAllForumTopicMessagesPostError>>;

    /// POST /unpinAllGeneralForumTopicMessages
    ///
    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn unpin_all_general_forum_topic_messages_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<UnpinAllGeneralForumTopicMessagesPostSuccess>, Error<UnpinAllGeneralForumTopicMessagesPostError>>;

    /// POST /unpinChatMessage
    ///
    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn unpin_chat_message_post<'chat_id, 'business_connection_id, 'message_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_id: Option<i32>) -> Result<ResponseContent<UnpinChatMessagePostSuccess>, Error<UnpinChatMessagePostError>>;

    /// POST /upgradeGift
    ///
    /// Upgrades a given regular gift to a unique gift. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Additionally requires the *can\\_transfer\\_stars* business bot right if the upgrade is paid. Returns *True* on success.
    async fn upgrade_gift_post<'business_connection_id, 'owned_gift_id, 'keep_original_details, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, keep_original_details: Option<bool>, star_count: Option<i32>) -> Result<ResponseContent<UpgradeGiftPostSuccess>, Error<UpgradeGiftPostError>>;

    /// POST /uploadStickerFile
    ///
    /// Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
    async fn upload_sticker_file_post<'user_id, 'sticker, 'sticker_format>(&self, user_id: i32, sticker: Option<models::serde_json::Value>, sticker_format: &'sticker_format str) -> Result<ResponseContent<UploadStickerFilePostSuccess>, Error<UploadStickerFilePostError>>;

    /// POST /verifyChat
    ///
    /// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn verify_chat_post<'chat_id, 'custom_description>(&self, chat_id: models::models::SendMessagePostRequestChatId, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<VerifyChatPostSuccess>, Error<VerifyChatPostError>>;

    /// POST /verifyUser
    ///
    /// Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn verify_user_post<'user_id, 'custom_description>(&self, user_id: i32, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<VerifyUserPostSuccess>, Error<VerifyUserPostError>>;
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
    async fn add_sticker_to_set_post<'user_id, 'name, 'sticker>(&self, user_id: i32, name: &'name str, sticker: models::models::InputSticker) -> Result<ResponseContent<AddStickerToSetPostSuccess>, Error<AddStickerToSetPostError>> {
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
            let local_var_entity: Option<AddStickerToSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddStickerToSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.  Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    async fn answer_callback_query_post<'callback_query_id, 'text, 'show_alert, 'url, 'cache_time>(&self, callback_query_id: &'callback_query_id str, text: Option<&'text str>, show_alert: Option<bool>, url: Option<&'url str>, cache_time: Option<i32>) -> Result<ResponseContent<AnswerCallbackQueryPostSuccess>, Error<AnswerCallbackQueryPostError>> {
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
            let local_var_entity: Option<AnswerCallbackQueryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AnswerCallbackQueryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send answers to an inline query. On success, *True* is returned.   No more than **50** results per query are allowed.
    async fn answer_inline_query_post<'inline_query_id, 'results, 'cache_time, 'is_personal, 'next_offset, 'button>(&self, inline_query_id: &'inline_query_id str, results: Vec<models::InlineQueryResult>, cache_time: Option<i32>, is_personal: Option<bool>, next_offset: Option<&'next_offset str>, button: Option<models::models::InlineQueryResultsButton>) -> Result<ResponseContent<AnswerInlineQueryPostSuccess>, Error<AnswerInlineQueryPostError>> {
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
            let local_var_entity: Option<AnswerInlineQueryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AnswerInlineQueryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\\_checkout\\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    async fn answer_pre_checkout_query_post<'pre_checkout_query_id, 'ok, 'error_message>(&self, pre_checkout_query_id: &'pre_checkout_query_id str, ok: bool, error_message: Option<&'error_message str>) -> Result<ResponseContent<AnswerPreCheckoutQueryPostSuccess>, Error<AnswerPreCheckoutQueryPostError>> {
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
            let local_var_entity: Option<AnswerPreCheckoutQueryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AnswerPreCheckoutQueryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// If you sent an invoice requesting a shipping address and the parameter *is\\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
    async fn answer_shipping_query_post<'shipping_query_id, 'ok, 'shipping_options, 'error_message>(&self, shipping_query_id: &'shipping_query_id str, ok: bool, shipping_options: Option<Vec<models::ShippingOption>>, error_message: Option<&'error_message str>) -> Result<ResponseContent<AnswerShippingQueryPostSuccess>, Error<AnswerShippingQueryPostError>> {
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
            let local_var_entity: Option<AnswerShippingQueryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AnswerShippingQueryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
    async fn answer_web_app_query_post<'web_app_query_id, 'result>(&self, web_app_query_id: &'web_app_query_id str, result: models::models::InlineQueryResult) -> Result<ResponseContent<AnswerWebAppQueryPostSuccess>, Error<AnswerWebAppQueryPostError>> {
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
            let local_var_entity: Option<AnswerWebAppQueryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AnswerWebAppQueryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn approve_chat_join_request_post<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, user_id: i32) -> Result<ResponseContent<ApproveChatJoinRequestPostSuccess>, Error<ApproveChatJoinRequestPostError>> {
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
            let local_var_entity: Option<ApproveChatJoinRequestPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ApproveChatJoinRequestPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn ban_chat_member_post<'chat_id, 'user_id, 'until_date, 'revoke_messages>(&self, chat_id: models::models::BanChatMemberPostRequestChatId, user_id: i32, until_date: Option<i32>, revoke_messages: Option<bool>) -> Result<ResponseContent<BanChatMemberPostSuccess>, Error<BanChatMemberPostError>> {
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
            let local_var_entity: Option<BanChatMemberPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BanChatMemberPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn ban_chat_sender_chat_post<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<BanChatSenderChatPostSuccess>, Error<BanChatSenderChatPostError>> {
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
            let local_var_entity: Option<BanChatSenderChatPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BanChatSenderChatPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn close_forum_topic_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<CloseForumTopicPostSuccess>, Error<CloseForumTopicPostError>> {
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
            let local_var_entity: Option<CloseForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CloseForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn close_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<CloseGeneralForumTopicPostSuccess>, Error<CloseGeneralForumTopicPostError>> {
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
            let local_var_entity: Option<CloseGeneralForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CloseGeneralForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
    async fn close_post<>(&self, ) -> Result<ResponseContent<ClosePostSuccess>, Error<ClosePostError>> {
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
            let local_var_entity: Option<ClosePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ClosePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Converts a given regular gift to Telegram Stars. Requires the *can\\_convert\\_gifts\\_to\\_stars* business bot right. Returns *True* on success.
    async fn convert_gift_to_stars_post<'business_connection_id, 'owned_gift_id>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str) -> Result<ResponseContent<ConvertGiftToStarsPostSuccess>, Error<ConvertGiftToStarsPostError>> {
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
            let local_var_entity: Option<ConvertGiftToStarsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ConvertGiftToStarsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api/#forwardmessage), but the copied message doesn't have a link to the original message. Returns the [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent message on success.
    async fn copy_message_post<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagePostRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<CopyMessagePostSuccess>, Error<CopyMessagePostError>> {
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
            let local_var_entity: Option<CopyMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CopyMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\\_option\\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn copy_messages_post<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content, 'remove_caption>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagesPostRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, remove_caption: Option<bool>) -> Result<ResponseContent<CopyMessagesPostSuccess>, Error<CopyMessagesPostError>> {
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
            let local_var_entity: Option<CopyMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CopyMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn create_chat_invite_link_post<'chat_id, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessagePostRequestChatId, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<CreateChatInviteLinkPostSuccess>, Error<CreateChatInviteLinkPostError>> {
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
            let local_var_entity: Option<CreateChatInviteLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateChatInviteLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\\_invite\\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn create_chat_subscription_invite_link_post<'chat_id, 'subscription_period, 'subscription_price, 'name>(&self, chat_id: models::models::CreateChatSubscriptionInviteLinkPostRequestChatId, subscription_period: i32, subscription_price: i32, name: Option<&'name str>) -> Result<ResponseContent<CreateChatSubscriptionInviteLinkPostSuccess>, Error<CreateChatSubscriptionInviteLinkPostError>> {
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
            let local_var_entity: Option<CreateChatSubscriptionInviteLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateChatSubscriptionInviteLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
    async fn create_forum_topic_post<'chat_id, 'name, 'icon_color, 'icon_custom_emoji_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, name: &'name str, icon_color: Option<i32>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<CreateForumTopicPostSuccess>, Error<CreateForumTopicPostError>> {
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
            let local_var_entity: Option<CreateForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
    async fn create_invoice_link_post<'title, 'description, 'payload, 'currency, 'prices, 'business_connection_id, 'provider_token, 'subscription_period, 'max_tip_amount, 'suggested_tip_amounts, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible>(&self, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, business_connection_id: Option<&'business_connection_id str>, provider_token: Option<&'provider_token str>, subscription_period: Option<i32>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>) -> Result<ResponseContent<CreateInvoiceLinkPostSuccess>, Error<CreateInvoiceLinkPostError>> {
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
            let local_var_entity: Option<CreateInvoiceLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateInvoiceLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
    async fn create_new_sticker_set_post<'user_id, 'name, 'title, 'stickers, 'sticker_type, 'needs_repainting>(&self, user_id: i32, name: &'name str, title: &'title str, stickers: Vec<models::InputSticker>, sticker_type: Option<&'sticker_type str>, needs_repainting: Option<bool>) -> Result<ResponseContent<CreateNewStickerSetPostSuccess>, Error<CreateNewStickerSetPostError>> {
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
            let local_var_entity: Option<CreateNewStickerSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateNewStickerSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\\_invite\\_users* administrator right. Returns *True* on success.
    async fn decline_chat_join_request_post<'chat_id, 'user_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, user_id: i32) -> Result<ResponseContent<DeclineChatJoinRequestPostSuccess>, Error<DeclineChatJoinRequestPostError>> {
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
            let local_var_entity: Option<DeclineChatJoinRequestPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeclineChatJoinRequestPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete messages on behalf of a business account. Requires the *can\\_delete\\_sent\\_messages* business bot right to delete messages sent by the bot itself, or the *can\\_delete\\_all\\_messages* business bot right to delete any message. Returns *True* on success.
    async fn delete_business_messages_post<'business_connection_id, 'message_ids>(&self, business_connection_id: &'business_connection_id str, message_ids: Vec<i32>) -> Result<ResponseContent<DeleteBusinessMessagesPostSuccess>, Error<DeleteBusinessMessagesPostError>> {
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
            let local_var_entity: Option<DeleteBusinessMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteBusinessMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn delete_chat_photo_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<DeleteChatPhotoPostSuccess>, Error<DeleteChatPhotoPostError>> {
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
            let local_var_entity: Option<DeleteChatPhotoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteChatPhotoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn delete_chat_sticker_set_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<DeleteChatStickerSetPostSuccess>, Error<DeleteChatStickerSetPostError>> {
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
            let local_var_entity: Option<DeleteChatStickerSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteChatStickerSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_delete\\_messages* administrator rights. Returns *True* on success.
    async fn delete_forum_topic_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<DeleteForumTopicPostSuccess>, Error<DeleteForumTopicPostError>> {
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
            let local_var_entity: Option<DeleteForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a message, including service messages, with the following limitations:   \\- A message can only be deleted if it was sent less than 48 hours ago.   \\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.   \\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.   \\- Bots can delete outgoing messages in private chats, groups, and supergroups.   \\- Bots can delete incoming messages in private chats.   \\- Bots granted *can\\_post\\_messages* permissions can delete outgoing messages in channels.   \\- If the bot is an administrator of a group, it can delete any message there.   \\- If the bot has *can\\_delete\\_messages* permission in a supergroup or a channel, it can delete any message there.   Returns *True* on success.
    async fn delete_message_post<'chat_id, 'message_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32) -> Result<ResponseContent<DeleteMessagePostSuccess>, Error<DeleteMessagePostError>> {
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
            let local_var_entity: Option<DeleteMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
    async fn delete_messages_post<'chat_id, 'message_ids>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_ids: Vec<i32>) -> Result<ResponseContent<DeleteMessagesPostSuccess>, Error<DeleteMessagesPostError>> {
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
            let local_var_entity: Option<DeleteMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
    async fn delete_my_commands_post<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<DeleteMyCommandsPostSuccess>, Error<DeleteMyCommandsPostError>> {
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
            let local_var_entity: Option<DeleteMyCommandsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteMyCommandsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
    async fn delete_sticker_from_set_post<'sticker>(&self, sticker: &'sticker str) -> Result<ResponseContent<DeleteStickerFromSetPostSuccess>, Error<DeleteStickerFromSetPostError>> {
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
            let local_var_entity: Option<DeleteStickerFromSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteStickerFromSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
    async fn delete_sticker_set_post<'name>(&self, name: &'name str) -> Result<ResponseContent<DeleteStickerSetPostSuccess>, Error<DeleteStickerSetPostError>> {
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
            let local_var_entity: Option<DeleteStickerSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteStickerSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns *True* on success.
    async fn delete_story_post<'business_connection_id, 'story_id>(&self, business_connection_id: &'business_connection_id str, story_id: i32) -> Result<ResponseContent<DeleteStoryPostSuccess>, Error<DeleteStoryPostError>> {
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
            let local_var_entity: Option<DeleteStoryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteStoryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
    async fn delete_webhook_post<'drop_pending_updates>(&self, drop_pending_updates: Option<bool>) -> Result<ResponseContent<DeleteWebhookPostSuccess>, Error<DeleteWebhookPostError>> {
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
            let local_var_entity: Option<DeleteWebhookPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteWebhookPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn edit_chat_invite_link_post<'chat_id, 'invite_link, 'name, 'expire_date, 'member_limit, 'creates_join_request>(&self, chat_id: models::models::SendMessagePostRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>, expire_date: Option<i32>, member_limit: Option<i32>, creates_join_request: Option<bool>) -> Result<ResponseContent<EditChatInviteLinkPostSuccess>, Error<EditChatInviteLinkPostError>> {
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
            let local_var_entity: Option<EditChatInviteLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditChatInviteLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit a subscription invite link created by the bot. The bot must have the *can\\_invite\\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn edit_chat_subscription_invite_link_post<'chat_id, 'invite_link, 'name>(&self, chat_id: models::models::SendMessagePostRequestChatId, invite_link: &'invite_link str, name: Option<&'name str>) -> Result<ResponseContent<EditChatSubscriptionInviteLinkPostSuccess>, Error<EditChatSubscriptionInviteLinkPostError>> {
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
            let local_var_entity: Option<EditChatSubscriptionInviteLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditChatSubscriptionInviteLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn edit_forum_topic_post<'chat_id, 'message_thread_id, 'name, 'icon_custom_emoji_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32, name: Option<&'name str>, icon_custom_emoji_id: Option<&'icon_custom_emoji_id str>) -> Result<ResponseContent<EditForumTopicPostSuccess>, Error<EditForumTopicPostError>> {
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
            let local_var_entity: Option<EditForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn edit_general_forum_topic_post<'chat_id, 'name>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, name: &'name str) -> Result<ResponseContent<EditGeneralForumTopicPostSuccess>, Error<EditGeneralForumTopicPostError>> {
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
            let local_var_entity: Option<EditGeneralForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditGeneralForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_caption_post<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageCaptionPostSuccess>, Error<EditMessageCaptionPostError>> {
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
            let local_var_entity: Option<EditMessageCaptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditMessageCaptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit live location messages. A location can be edited until its *live\\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn edit_message_live_location_post<'latitude, 'longitude, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'live_period, 'horizontal_accuracy, 'heading, 'proximity_alert_radius, 'reply_markup>(&self, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, live_period: Option<i32>, horizontal_accuracy: Option<f64>, heading: Option<i32>, proximity_alert_radius: Option<i32>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageLiveLocationPostSuccess>, Error<EditMessageLiveLocationPostError>> {
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
            let local_var_entity: Option<EditMessageLiveLocationPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditMessageLiveLocationPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_media_post<'media, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, media: models::models::InputMedia, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageMediaPostSuccess>, Error<EditMessageMediaPostError>> {
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
            let local_var_entity: Option<EditMessageMediaPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditMessageMediaPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_reply_markup_post<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageReplyMarkupPostSuccess>, Error<EditMessageReplyMarkupPostError>> {
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
            let local_var_entity: Option<EditMessageReplyMarkupPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditMessageReplyMarkupPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    async fn edit_message_text_post<'text, 'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'parse_mode, 'entities, 'link_preview_options, 'reply_markup>(&self, text: &'text str, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<EditMessageTextPostSuccess>, Error<EditMessageTextPostError>> {
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
            let local_var_entity: Option<EditMessageTextPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditMessageTextPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Edits a story previously posted by the bot on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn edit_story_post<'business_connection_id, 'story_id, 'content, 'caption, 'parse_mode, 'caption_entities, 'areas>(&self, business_connection_id: &'business_connection_id str, story_id: i32, content: models::models::InputStoryContent, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>) -> Result<ResponseContent<EditStoryPostSuccess>, Error<EditStoryPostError>> {
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
            let local_var_entity: Option<EditStoryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditStoryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
    async fn edit_user_star_subscription_post<'user_id, 'telegram_payment_charge_id, 'is_canceled>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str, is_canceled: bool) -> Result<ResponseContent<EditUserStarSubscriptionPostSuccess>, Error<EditUserStarSubscriptionPostError>> {
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
            let local_var_entity: Option<EditUserStarSubscriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditUserStarSubscriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
    async fn export_chat_invite_link_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<ExportChatInviteLinkPostSuccess>, Error<ExportChatInviteLinkPostError>> {
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
            let local_var_entity: Option<ExportChatInviteLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ExportChatInviteLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn forward_message_post<'chat_id, 'from_chat_id, 'message_id, 'message_thread_id, 'video_start_timestamp, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagePostRequestFromChatId, message_id: i32, message_thread_id: Option<i32>, video_start_timestamp: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<ForwardMessagePostSuccess>, Error<ForwardMessagePostError>> {
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
            let local_var_entity: Option<ForwardMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ForwardMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    async fn forward_messages_post<'chat_id, 'from_chat_id, 'message_ids, 'message_thread_id, 'disable_notification, 'protect_content>(&self, chat_id: models::models::SendMessagePostRequestChatId, from_chat_id: models::models::ForwardMessagesPostRequestFromChatId, message_ids: Vec<i32>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<ForwardMessagesPostSuccess>, Error<ForwardMessagesPostError>> {
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
            let local_var_entity: Option<ForwardMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ForwardMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
    async fn get_available_gifts_post<>(&self, ) -> Result<ResponseContent<GetAvailableGiftsPostSuccess>, Error<GetAvailableGiftsPostError>> {
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
            let local_var_entity: Option<GetAvailableGiftsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetAvailableGiftsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the gifts received and owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [OwnedGifts](https://core.telegram.org/bots/api/#ownedgifts) on success.
    async fn get_business_account_gifts_post<'business_connection_id, 'exclude_unsaved, 'exclude_saved, 'exclude_unlimited, 'exclude_limited, 'exclude_unique, 'sort_by_price, 'offset, 'limit>(&self, business_connection_id: &'business_connection_id str, exclude_unsaved: Option<bool>, exclude_saved: Option<bool>, exclude_unlimited: Option<bool>, exclude_limited: Option<bool>, exclude_unique: Option<bool>, sort_by_price: Option<bool>, offset: Option<&'offset str>, limit: Option<i32>) -> Result<ResponseContent<GetBusinessAccountGiftsPostSuccess>, Error<GetBusinessAccountGiftsPostError>> {
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
            let local_var_entity: Option<GetBusinessAccountGiftsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetBusinessAccountGiftsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the *can\\_view\\_gifts\\_and\\_stars* business bot right. Returns [StarAmount](https://core.telegram.org/bots/api/#staramount) on success.
    async fn get_business_account_star_balance_post<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<GetBusinessAccountStarBalancePostSuccess>, Error<GetBusinessAccountStarBalancePostError>> {
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
            let local_var_entity: Option<GetBusinessAccountStarBalancePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetBusinessAccountStarBalancePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
    async fn get_business_connection_post<'business_connection_id>(&self, business_connection_id: &'business_connection_id str) -> Result<ResponseContent<GetBusinessConnectionPostSuccess>, Error<GetBusinessConnectionPostError>> {
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
            let local_var_entity: Option<GetBusinessConnectionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetBusinessConnectionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
    async fn get_chat_administrators_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<GetChatAdministratorsPostSuccess>, Error<GetChatAdministratorsPostError>> {
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
            let local_var_entity: Option<GetChatAdministratorsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetChatAdministratorsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the number of members in a chat. Returns *Int* on success.
    async fn get_chat_member_count_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<GetChatMemberCountPostSuccess>, Error<GetChatMemberCountPostError>> {
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
            let local_var_entity: Option<GetChatMemberCountPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetChatMemberCountPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
    async fn get_chat_member_post<'chat_id, 'user_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId, user_id: i32) -> Result<ResponseContent<GetChatMemberPostSuccess>, Error<GetChatMemberPostError>> {
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
            let local_var_entity: Option<GetChatMemberPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetChatMemberPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
    async fn get_chat_menu_button_post<'chat_id>(&self, chat_id: Option<i32>) -> Result<ResponseContent<GetChatMenuButtonPostSuccess>, Error<GetChatMenuButtonPostError>> {
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
            let local_var_entity: Option<GetChatMenuButtonPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetChatMenuButtonPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
    async fn get_chat_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<GetChatPostSuccess>, Error<GetChatPostError>> {
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
            let local_var_entity: Option<GetChatPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetChatPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn get_custom_emoji_stickers_post<'custom_emoji_ids>(&self, custom_emoji_ids: Vec<String>) -> Result<ResponseContent<GetCustomEmojiStickersPostSuccess>, Error<GetCustomEmojiStickersPostError>> {
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
            let local_var_entity: Option<GetCustomEmojiStickersPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetCustomEmojiStickersPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
    async fn get_file_post<'file_id>(&self, file_id: &'file_id str) -> Result<ResponseContent<GetFilePostSuccess>, Error<GetFilePostError>> {
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
            let local_var_entity: Option<GetFilePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetFilePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    async fn get_forum_topic_icon_stickers_post<>(&self, ) -> Result<ResponseContent<GetForumTopicIconStickersPostSuccess>, Error<GetForumTopicIconStickersPostError>> {
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
            let local_var_entity: Option<GetForumTopicIconStickersPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetForumTopicIconStickersPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.  This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
    async fn get_game_high_scores_post<'user_id, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<GetGameHighScoresPostSuccess>, Error<GetGameHighScoresPostError>> {
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
            let local_var_entity: Option<GetGameHighScoresPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGameHighScoresPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
    async fn get_me_post<>(&self, ) -> Result<ResponseContent<GetMePostSuccess>, Error<GetMePostError>> {
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
            let local_var_entity: Option<GetMePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
    async fn get_my_commands_post<'scope, 'language_code>(&self, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyCommandsPostSuccess>, Error<GetMyCommandsPostError>> {
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
            let local_var_entity: Option<GetMyCommandsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyCommandsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
    async fn get_my_default_administrator_rights_post<'for_channels>(&self, for_channels: Option<bool>) -> Result<ResponseContent<GetMyDefaultAdministratorRightsPostSuccess>, Error<GetMyDefaultAdministratorRightsPostError>> {
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
            let local_var_entity: Option<GetMyDefaultAdministratorRightsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyDefaultAdministratorRightsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
    async fn get_my_description_post<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyDescriptionPostSuccess>, Error<GetMyDescriptionPostError>> {
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
            let local_var_entity: Option<GetMyDescriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyDescriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
    async fn get_my_name_post<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyNamePostSuccess>, Error<GetMyNamePostError>> {
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
            let local_var_entity: Option<GetMyNamePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyNamePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
    async fn get_my_short_description_post<'language_code>(&self, language_code: Option<&'language_code str>) -> Result<ResponseContent<GetMyShortDescriptionPostSuccess>, Error<GetMyShortDescriptionPostError>> {
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
            let local_var_entity: Option<GetMyShortDescriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyShortDescriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
    async fn get_star_transactions_post<'offset, 'limit>(&self, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<GetStarTransactionsPostSuccess>, Error<GetStarTransactionsPostError>> {
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
            let local_var_entity: Option<GetStarTransactionsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetStarTransactionsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
    async fn get_sticker_set_post<'name>(&self, name: &'name str) -> Result<ResponseContent<GetStickerSetPostSuccess>, Error<GetStickerSetPostError>> {
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
            let local_var_entity: Option<GetStickerSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetStickerSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.
    async fn get_updates_post<'offset, 'limit, 'timeout, 'allowed_updates>(&self, offset: Option<i32>, limit: Option<i32>, timeout: Option<i32>, allowed_updates: Option<Vec<String>>) -> Result<ResponseContent<GetUpdatesPostSuccess>, Error<GetUpdatesPostError>> {
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
            let local_var_entity: Option<GetUpdatesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetUpdatesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
    async fn get_user_chat_boosts_post<'chat_id, 'user_id>(&self, chat_id: models::models::GetUserChatBoostsPostRequestChatId, user_id: i32) -> Result<ResponseContent<GetUserChatBoostsPostSuccess>, Error<GetUserChatBoostsPostError>> {
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
            let local_var_entity: Option<GetUserChatBoostsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetUserChatBoostsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
    async fn get_user_profile_photos_post<'user_id, 'offset, 'limit>(&self, user_id: i32, offset: Option<i32>, limit: Option<i32>) -> Result<ResponseContent<GetUserProfilePhotosPostSuccess>, Error<GetUserProfilePhotosPostError>> {
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
            let local_var_entity: Option<GetUserProfilePhotosPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetUserProfilePhotosPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
    async fn get_webhook_info_post<>(&self, ) -> Result<ResponseContent<GetWebhookInfoPostSuccess>, Error<GetWebhookInfoPostError>> {
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
            let local_var_entity: Option<GetWebhookInfoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetWebhookInfoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Gifts a Telegram Premium subscription to the given user. Returns *True* on success.
    async fn gift_premium_subscription_post<'user_id, 'month_count, 'star_count, 'text, 'text_parse_mode, 'text_entities>(&self, user_id: i32, month_count: i32, star_count: i32, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<GiftPremiumSubscriptionPostSuccess>, Error<GiftPremiumSubscriptionPostError>> {
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
            let local_var_entity: Option<GiftPremiumSubscriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GiftPremiumSubscriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
    async fn hide_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<HideGeneralForumTopicPostSuccess>, Error<HideGeneralForumTopicPostError>> {
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
            let local_var_entity: Option<HideGeneralForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<HideGeneralForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    async fn leave_chat_post<'chat_id>(&self, chat_id: models::models::LeaveChatPostRequestChatId) -> Result<ResponseContent<LeaveChatPostSuccess>, Error<LeaveChatPostError>> {
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
            let local_var_entity: Option<LeaveChatPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<LeaveChatPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
    async fn log_out_post<>(&self, ) -> Result<ResponseContent<LogOutPostSuccess>, Error<LogOutPostError>> {
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
            let local_var_entity: Option<LogOutPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<LogOutPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn pin_chat_message_post<'chat_id, 'message_id, 'business_connection_id, 'disable_notification>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, disable_notification: Option<bool>) -> Result<ResponseContent<PinChatMessagePostSuccess>, Error<PinChatMessagePostError>> {
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
            let local_var_entity: Option<PinChatMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PinChatMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Posts a story on behalf of a managed business account. Requires the *can\\_manage\\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
    async fn post_story_post<'business_connection_id, 'content, 'active_period, 'caption, 'parse_mode, 'caption_entities, 'areas, 'post_to_chat_page, 'protect_content>(&self, business_connection_id: &'business_connection_id str, content: models::models::InputStoryContent, active_period: i32, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, areas: Option<Vec<models::StoryArea>>, post_to_chat_page: Option<bool>, protect_content: Option<bool>) -> Result<ResponseContent<PostStoryPostSuccess>, Error<PostStoryPostError>> {
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
            let local_var_entity: Option<PostStoryPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PostStoryPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    async fn promote_chat_member_post<'chat_id, 'user_id, 'is_anonymous, 'can_manage_chat, 'can_delete_messages, 'can_manage_video_chats, 'can_restrict_members, 'can_promote_members, 'can_change_info, 'can_invite_users, 'can_post_stories, 'can_edit_stories, 'can_delete_stories, 'can_post_messages, 'can_edit_messages, 'can_pin_messages, 'can_manage_topics>(&self, chat_id: models::models::SendMessagePostRequestChatId, user_id: i32, is_anonymous: Option<bool>, can_manage_chat: Option<bool>, can_delete_messages: Option<bool>, can_manage_video_chats: Option<bool>, can_restrict_members: Option<bool>, can_promote_members: Option<bool>, can_change_info: Option<bool>, can_invite_users: Option<bool>, can_post_stories: Option<bool>, can_edit_stories: Option<bool>, can_delete_stories: Option<bool>, can_post_messages: Option<bool>, can_edit_messages: Option<bool>, can_pin_messages: Option<bool>, can_manage_topics: Option<bool>) -> Result<ResponseContent<PromoteChatMemberPostSuccess>, Error<PromoteChatMemberPostError>> {
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
            let local_var_entity: Option<PromoteChatMemberPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PromoteChatMemberPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Marks incoming message as read on behalf of a business account. Requires the *can\\_read\\_messages* business bot right. Returns *True* on success.
    async fn read_business_message_post<'business_connection_id, 'chat_id, 'message_id>(&self, business_connection_id: &'business_connection_id str, chat_id: i32, message_id: i32) -> Result<ResponseContent<ReadBusinessMessagePostSuccess>, Error<ReadBusinessMessagePostError>> {
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
            let local_var_entity: Option<ReadBusinessMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ReadBusinessMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
    async fn refund_star_payment_post<'user_id, 'telegram_payment_charge_id>(&self, user_id: i32, telegram_payment_charge_id: &'telegram_payment_charge_id str) -> Result<ResponseContent<RefundStarPaymentPostSuccess>, Error<RefundStarPaymentPostError>> {
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
            let local_var_entity: Option<RefundStarPaymentPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<RefundStarPaymentPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Removes the current profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn remove_business_account_profile_photo_post<'business_connection_id, 'is_public>(&self, business_connection_id: &'business_connection_id str, is_public: Option<bool>) -> Result<ResponseContent<RemoveBusinessAccountProfilePhotoPostSuccess>, Error<RemoveBusinessAccountProfilePhotoPostError>> {
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
            let local_var_entity: Option<RemoveBusinessAccountProfilePhotoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<RemoveBusinessAccountProfilePhotoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn remove_chat_verification_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<RemoveChatVerificationPostSuccess>, Error<RemoveChatVerificationPostError>> {
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
            let local_var_entity: Option<RemoveChatVerificationPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<RemoveChatVerificationPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
    async fn remove_user_verification_post<'user_id>(&self, user_id: i32) -> Result<ResponseContent<RemoveUserVerificationPostSuccess>, Error<RemoveUserVerificationPostError>> {
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
            let local_var_entity: Option<RemoveUserVerificationPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<RemoveUserVerificationPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    async fn reopen_forum_topic_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<ReopenForumTopicPostSuccess>, Error<ReopenForumTopicPostError>> {
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
            let local_var_entity: Option<ReopenForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ReopenForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
    async fn reopen_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<ReopenGeneralForumTopicPostSuccess>, Error<ReopenGeneralForumTopicPostError>> {
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
            let local_var_entity: Option<ReopenGeneralForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ReopenGeneralForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
    async fn replace_sticker_in_set_post<'user_id, 'name, 'old_sticker, 'sticker>(&self, user_id: i32, name: &'name str, old_sticker: &'old_sticker str, sticker: models::models::InputSticker) -> Result<ResponseContent<ReplaceStickerInSetPostSuccess>, Error<ReplaceStickerInSetPostError>> {
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
            let local_var_entity: Option<ReplaceStickerInSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ReplaceStickerInSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
    async fn restrict_chat_member_post<'chat_id, 'user_id, 'permissions, 'use_independent_chat_permissions, 'until_date>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, user_id: i32, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>, until_date: Option<i32>) -> Result<ResponseContent<RestrictChatMemberPostSuccess>, Error<RestrictChatMemberPostError>> {
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
            let local_var_entity: Option<RestrictChatMemberPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<RestrictChatMemberPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    async fn revoke_chat_invite_link_post<'chat_id, 'invite_link>(&self, chat_id: models::models::RevokeChatInviteLinkPostRequestChatId, invite_link: &'invite_link str) -> Result<ResponseContent<RevokeChatInviteLinkPostSuccess>, Error<RevokeChatInviteLinkPostError>> {
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
            let local_var_entity: Option<RevokeChatInviteLinkPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<RevokeChatInviteLinkPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
    async fn save_prepared_inline_message_post<'user_id, 'result, 'allow_user_chats, 'allow_bot_chats, 'allow_group_chats, 'allow_channel_chats>(&self, user_id: i32, result: models::models::InlineQueryResult, allow_user_chats: Option<bool>, allow_bot_chats: Option<bool>, allow_group_chats: Option<bool>, allow_channel_chats: Option<bool>) -> Result<ResponseContent<SavePreparedInlineMessagePostSuccess>, Error<SavePreparedInlineMessagePostError>> {
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
            let local_var_entity: Option<SavePreparedInlineMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SavePreparedInlineMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    async fn send_animation_post<'chat_id, 'animation, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, animation: models::models::SendAnimationPostRequestAnimation, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendAnimationPostSuccess>, Error<SendAnimationPostError>> {
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
        local_var_form = local_var_form.text("animation", animation.to_string());
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
            let local_var_entity: Option<SendAnimationPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendAnimationPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.  For sending voice messages, use the [sendVoice](https://core.telegram.org/bots/api/#sendvoice) method instead.
    async fn send_audio_post<'chat_id, 'audio, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'performer, 'title, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, audio: models::models::SendAudioPostRequestAudio, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, performer: Option<&'performer str>, title: Option<&'title str>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendAudioPostSuccess>, Error<SendAudioPostError>> {
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
        local_var_form = local_var_form.text("audio", audio.to_string());
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
            let local_var_entity: Option<SendAudioPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendAudioPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.  Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\\_photo*. The user will see a “sending photo” status for the bot.  We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    async fn send_chat_action_post<'chat_id, 'action, 'business_connection_id, 'message_thread_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, action: &'action str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>) -> Result<ResponseContent<SendChatActionPostSuccess>, Error<SendChatActionPostError>> {
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
            let local_var_entity: Option<SendChatActionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendChatActionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send phone contacts. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_contact_post<'chat_id, 'phone_number, 'first_name, 'business_connection_id, 'message_thread_id, 'last_name, 'vcard, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, phone_number: &'phone_number str, first_name: &'first_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, last_name: Option<&'last_name str>, vcard: Option<&'vcard str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendContactPostSuccess>, Error<SendContactPostError>> {
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
            let local_var_entity: Option<SendContactPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendContactPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_dice_post<'chat_id, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendDicePostSuccess>, Error<SendDicePostError>> {
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
            let local_var_entity: Option<SendDicePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendDicePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send general files. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    async fn send_document_post<'chat_id, 'document, 'business_connection_id, 'message_thread_id, 'thumbnail, 'caption, 'parse_mode, 'caption_entities, 'disable_content_type_detection, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, document: models::models::SendDocumentPostRequestDocument, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, disable_content_type_detection: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendDocumentPostSuccess>, Error<SendDocumentPostError>> {
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
        local_var_form = local_var_form.text("document", document.to_string());
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
            let local_var_entity: Option<SendDocumentPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendDocumentPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_game_post<'chat_id, 'game_short_name, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: i32, game_short_name: &'game_short_name str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<SendGamePostSuccess>, Error<SendGamePostError>> {
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
            let local_var_entity: Option<SendGamePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendGamePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.
    async fn send_gift_post<'gift_id, 'user_id, 'chat_id, 'pay_for_upgrade, 'text, 'text_parse_mode, 'text_entities>(&self, gift_id: &'gift_id str, user_id: Option<i32>, chat_id: Option<models::models::SendGiftPostRequestChatId>, pay_for_upgrade: Option<bool>, text: Option<&'text str>, text_parse_mode: Option<&'text_parse_mode str>, text_entities: Option<Vec<models::MessageEntity>>) -> Result<ResponseContent<SendGiftPostSuccess>, Error<SendGiftPostError>> {
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
            let local_var_entity: Option<SendGiftPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendGiftPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_invoice_post<'chat_id, 'title, 'description, 'payload, 'currency, 'prices, 'message_thread_id, 'provider_token, 'max_tip_amount, 'suggested_tip_amounts, 'start_parameter, 'provider_data, 'photo_url, 'photo_size, 'photo_width, 'photo_height, 'need_name, 'need_phone_number, 'need_email, 'need_shipping_address, 'send_phone_number_to_provider, 'send_email_to_provider, 'is_flexible, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, title: &'title str, description: &'description str, payload: &'payload str, currency: &'currency str, prices: Vec<models::LabeledPrice>, message_thread_id: Option<i32>, provider_token: Option<&'provider_token str>, max_tip_amount: Option<i32>, suggested_tip_amounts: Option<Vec<i32>>, start_parameter: Option<&'start_parameter str>, provider_data: Option<&'provider_data str>, photo_url: Option<&'photo_url str>, photo_size: Option<i32>, photo_width: Option<i32>, photo_height: Option<i32>, need_name: Option<bool>, need_phone_number: Option<bool>, need_email: Option<bool>, need_shipping_address: Option<bool>, send_phone_number_to_provider: Option<bool>, send_email_to_provider: Option<bool>, is_flexible: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<SendInvoicePostSuccess>, Error<SendInvoicePostError>> {
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
            let local_var_entity: Option<SendInvoicePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendInvoicePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_location_post<'chat_id, 'latitude, 'longitude, 'business_connection_id, 'message_thread_id, 'horizontal_accuracy, 'live_period, 'heading, 'proximity_alert_radius, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, latitude: f64, longitude: f64, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, horizontal_accuracy: Option<f64>, live_period: Option<i32>, heading: Option<i32>, proximity_alert_radius: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendLocationPostSuccess>, Error<SendLocationPostError>> {
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
            let local_var_entity: Option<SendLocationPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendLocationPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
    async fn send_media_group_post<'chat_id, 'media, 'business_connection_id, 'message_thread_id, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters>(&self, chat_id: models::models::SendMessagePostRequestChatId, media: Vec<models::SendMediaGroupPostRequestMediaInner>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>) -> Result<ResponseContent<SendMediaGroupPostSuccess>, Error<SendMediaGroupPostError>> {
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
            let local_var_entity: Option<SendMediaGroupPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendMediaGroupPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_message_post<'chat_id, 'text, 'business_connection_id, 'message_thread_id, 'parse_mode, 'entities, 'link_preview_options, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, text: &'text str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, parse_mode: Option<&'parse_mode str>, entities: Option<Vec<models::MessageEntity>>, link_preview_options: Option<models::models::LinkPreviewOptions>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendMessagePostSuccess>, Error<SendMessagePostError>> {
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
            let local_var_entity: Option<SendMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_paid_media_post<'chat_id, 'star_count, 'media, 'business_connection_id, 'payload, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendPaidMediaPostRequestChatId, star_count: i32, media: Vec<models::InputPaidMedia>, business_connection_id: Option<&'business_connection_id str>, payload: Option<&'payload str>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendPaidMediaPostSuccess>, Error<SendPaidMediaPostError>> {
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
            let local_var_entity: Option<SendPaidMediaPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendPaidMediaPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send photos. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_photo_post<'chat_id, 'photo, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, photo: models::models::SendPhotoPostRequestPhoto, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendPhotoPostSuccess>, Error<SendPhotoPostError>> {
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
        local_var_form = local_var_form.text("photo", photo.to_string());
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
            let local_var_entity: Option<SendPhotoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendPhotoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_poll_post<'chat_id, 'question, 'options, 'business_connection_id, 'message_thread_id, 'question_parse_mode, 'question_entities, 'is_anonymous, 'r_type, 'allows_multiple_answers, 'correct_option_id, 'explanation, 'explanation_parse_mode, 'explanation_entities, 'open_period, 'close_date, 'is_closed, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, question: &'question str, options: Vec<models::InputPollOption>, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, question_parse_mode: Option<&'question_parse_mode str>, question_entities: Option<Vec<models::MessageEntity>>, is_anonymous: Option<bool>, r#type: Option<&'r_type str>, allows_multiple_answers: Option<bool>, correct_option_id: Option<i32>, explanation: Option<&'explanation str>, explanation_parse_mode: Option<&'explanation_parse_mode str>, explanation_entities: Option<Vec<models::MessageEntity>>, open_period: Option<i32>, close_date: Option<i32>, is_closed: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendPollPostSuccess>, Error<SendPollPostError>> {
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
            let local_var_entity: Option<SendPollPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendPollPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_sticker_post<'chat_id, 'sticker, 'business_connection_id, 'message_thread_id, 'emoji, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, sticker: models::models::SendStickerPostRequestSticker, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, emoji: Option<&'emoji str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendStickerPostSuccess>, Error<SendStickerPostError>> {
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
        local_var_form = local_var_form.text("sticker", sticker.to_string());
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
            let local_var_entity: Option<SendStickerPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendStickerPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_venue_post<'chat_id, 'latitude, 'longitude, 'title, 'address, 'business_connection_id, 'message_thread_id, 'foursquare_id, 'foursquare_type, 'google_place_id, 'google_place_type, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, latitude: f64, longitude: f64, title: &'title str, address: &'address str, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, foursquare_id: Option<&'foursquare_id str>, foursquare_type: Option<&'foursquare_type str>, google_place_id: Option<&'google_place_id str>, google_place_type: Option<&'google_place_type str>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVenuePostSuccess>, Error<SendVenuePostError>> {
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
            let local_var_entity: Option<SendVenuePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendVenuePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    async fn send_video_note_post<'chat_id, 'video_note, 'business_connection_id, 'message_thread_id, 'duration, 'length, 'thumbnail, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, video_note: models::models::SendVideoNotePostRequestVideoNote, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, length: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVideoNotePostSuccess>, Error<SendVideoNotePostError>> {
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
        local_var_form = local_var_form.text("video_note", video_note.to_string());
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
            let local_var_entity: Option<SendVideoNotePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendVideoNotePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    async fn send_video_post<'chat_id, 'video, 'business_connection_id, 'message_thread_id, 'duration, 'width, 'height, 'thumbnail, 'cover, 'start_timestamp, 'caption, 'parse_mode, 'caption_entities, 'show_caption_above_media, 'has_spoiler, 'supports_streaming, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, video: models::models::SendVideoPostRequestVideo, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, duration: Option<i32>, width: Option<i32>, height: Option<i32>, thumbnail: Option<models::models::SendAudioPostRequestThumbnail>, cover: Option<models::models::SendVideoPostRequestCover>, start_timestamp: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, show_caption_above_media: Option<bool>, has_spoiler: Option<bool>, supports_streaming: Option<bool>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVideoPostSuccess>, Error<SendVideoPostError>> {
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
        local_var_form = local_var_form.text("video", video.to_string());
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
            let local_var_entity: Option<SendVideoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendVideoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as [Audio](https://core.telegram.org/bots/api/#audio) or [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    async fn send_voice_post<'chat_id, 'voice, 'business_connection_id, 'message_thread_id, 'caption, 'parse_mode, 'caption_entities, 'duration, 'disable_notification, 'protect_content, 'allow_paid_broadcast, 'message_effect_id, 'reply_parameters, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, voice: models::models::SendVoicePostRequestVoice, business_connection_id: Option<&'business_connection_id str>, message_thread_id: Option<i32>, caption: Option<&'caption str>, parse_mode: Option<&'parse_mode str>, caption_entities: Option<Vec<models::MessageEntity>>, duration: Option<i32>, disable_notification: Option<bool>, protect_content: Option<bool>, allow_paid_broadcast: Option<bool>, message_effect_id: Option<&'message_effect_id str>, reply_parameters: Option<models::models::ReplyParameters>, reply_markup: Option<models::models::SendMessagePostRequestReplyMarkup>) -> Result<ResponseContent<SendVoicePostSuccess>, Error<SendVoicePostError>> {
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
        local_var_form = local_var_form.text("voice", voice.to_string());
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
            let local_var_entity: Option<SendVoicePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendVoicePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the bio of a managed business account. Requires the *can\\_change\\_bio* business bot right. Returns *True* on success.
    async fn set_business_account_bio_post<'business_connection_id, 'bio>(&self, business_connection_id: &'business_connection_id str, bio: Option<&'bio str>) -> Result<ResponseContent<SetBusinessAccountBioPostSuccess>, Error<SetBusinessAccountBioPostError>> {
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
            let local_var_entity: Option<SetBusinessAccountBioPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetBusinessAccountBioPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the *can\\_change\\_gift\\_settings* business bot right. Returns *True* on success.
    async fn set_business_account_gift_settings_post<'business_connection_id, 'show_gift_button, 'accepted_gift_types>(&self, business_connection_id: &'business_connection_id str, show_gift_button: bool, accepted_gift_types: models::models::AcceptedGiftTypes) -> Result<ResponseContent<SetBusinessAccountGiftSettingsPostSuccess>, Error<SetBusinessAccountGiftSettingsPostError>> {
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
            let local_var_entity: Option<SetBusinessAccountGiftSettingsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetBusinessAccountGiftSettingsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the first and last name of a managed business account. Requires the *can\\_change\\_name* business bot right. Returns *True* on success.
    async fn set_business_account_name_post<'business_connection_id, 'first_name, 'last_name>(&self, business_connection_id: &'business_connection_id str, first_name: &'first_name str, last_name: Option<&'last_name str>) -> Result<ResponseContent<SetBusinessAccountNamePostSuccess>, Error<SetBusinessAccountNamePostError>> {
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
            let local_var_entity: Option<SetBusinessAccountNamePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetBusinessAccountNamePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the profile photo of a managed business account. Requires the *can\\_edit\\_profile\\_photo* business bot right. Returns *True* on success.
    async fn set_business_account_profile_photo_post<'business_connection_id, 'photo, 'is_public>(&self, business_connection_id: &'business_connection_id str, photo: models::models::InputProfilePhoto, is_public: Option<bool>) -> Result<ResponseContent<SetBusinessAccountProfilePhotoPostSuccess>, Error<SetBusinessAccountProfilePhotoPostError>> {
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
            let local_var_entity: Option<SetBusinessAccountProfilePhotoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetBusinessAccountProfilePhotoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the username of a managed business account. Requires the *can\\_change\\_username* business bot right. Returns *True* on success.
    async fn set_business_account_username_post<'business_connection_id, 'username>(&self, business_connection_id: &'business_connection_id str, username: Option<&'username str>) -> Result<ResponseContent<SetBusinessAccountUsernamePostSuccess>, Error<SetBusinessAccountUsernamePostError>> {
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
            let local_var_entity: Option<SetBusinessAccountUsernamePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetBusinessAccountUsernamePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
    async fn set_chat_administrator_custom_title_post<'chat_id, 'user_id, 'custom_title>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, user_id: i32, custom_title: &'custom_title str) -> Result<ResponseContent<SetChatAdministratorCustomTitlePostSuccess>, Error<SetChatAdministratorCustomTitlePostError>> {
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
            let local_var_entity: Option<SetChatAdministratorCustomTitlePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatAdministratorCustomTitlePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn set_chat_description_post<'chat_id, 'description>(&self, chat_id: models::models::SendMessagePostRequestChatId, description: Option<&'description str>) -> Result<ResponseContent<SetChatDescriptionPostSuccess>, Error<SetChatDescriptionPostError>> {
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
            let local_var_entity: Option<SetChatDescriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatDescriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
    async fn set_chat_menu_button_post<'chat_id, 'menu_button>(&self, chat_id: Option<i32>, menu_button: Option<models::models::MenuButton>) -> Result<ResponseContent<SetChatMenuButtonPostSuccess>, Error<SetChatMenuButtonPostError>> {
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
            let local_var_entity: Option<SetChatMenuButtonPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatMenuButtonPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\\_restrict\\_members* administrator rights. Returns *True* on success.
    async fn set_chat_permissions_post<'chat_id, 'permissions, 'use_independent_chat_permissions>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, permissions: models::models::ChatPermissions, use_independent_chat_permissions: Option<bool>) -> Result<ResponseContent<SetChatPermissionsPostSuccess>, Error<SetChatPermissionsPostError>> {
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
            let local_var_entity: Option<SetChatPermissionsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatPermissionsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn set_chat_photo_post<'chat_id, 'photo>(&self, chat_id: models::models::SendMessagePostRequestChatId, photo: Option<models::serde_json::Value>) -> Result<ResponseContent<SetChatPhotoPostSuccess>, Error<SetChatPhotoPostError>> {
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
            let local_var_entity: Option<SetChatPhotoPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatPhotoPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\\_set\\_sticker\\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    async fn set_chat_sticker_set_post<'chat_id, 'sticker_set_name>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, sticker_set_name: &'sticker_set_name str) -> Result<ResponseContent<SetChatStickerSetPostSuccess>, Error<SetChatStickerSetPostError>> {
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
            let local_var_entity: Option<SetChatStickerSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatStickerSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn set_chat_title_post<'chat_id, 'title>(&self, chat_id: models::models::SendMessagePostRequestChatId, title: &'title str) -> Result<ResponseContent<SetChatTitlePostSuccess>, Error<SetChatTitlePostError>> {
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
            let local_var_entity: Option<SetChatTitlePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChatTitlePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
    async fn set_custom_emoji_sticker_set_thumbnail_post<'name, 'custom_emoji_id>(&self, name: &'name str, custom_emoji_id: Option<&'custom_emoji_id str>) -> Result<ResponseContent<SetCustomEmojiStickerSetThumbnailPostSuccess>, Error<SetCustomEmojiStickerSetThumbnailPostError>> {
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
            let local_var_entity: Option<SetCustomEmojiStickerSetThumbnailPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetCustomEmojiStickerSetThumbnailPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
    async fn set_game_score_post<'user_id, 'score, 'force, 'disable_edit_message, 'chat_id, 'message_id, 'inline_message_id>(&self, user_id: i32, score: i32, force: Option<bool>, disable_edit_message: Option<bool>, chat_id: Option<i32>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>) -> Result<ResponseContent<SetGameScorePostSuccess>, Error<SetGameScorePostError>> {
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
            let local_var_entity: Option<SetGameScorePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetGameScorePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.
    async fn set_message_reaction_post<'chat_id, 'message_id, 'reaction, 'is_big>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32, reaction: Option<Vec<models::ReactionType>>, is_big: Option<bool>) -> Result<ResponseContent<SetMessageReactionPostSuccess>, Error<SetMessageReactionPostError>> {
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
            let local_var_entity: Option<SetMessageReactionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetMessageReactionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
    async fn set_my_commands_post<'commands, 'scope, 'language_code>(&self, commands: Vec<models::BotCommand>, scope: Option<models::models::BotCommandScope>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyCommandsPostSuccess>, Error<SetMyCommandsPostError>> {
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
            let local_var_entity: Option<SetMyCommandsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetMyCommandsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
    async fn set_my_default_administrator_rights_post<'rights, 'for_channels>(&self, rights: Option<models::models::ChatAdministratorRights>, for_channels: Option<bool>) -> Result<ResponseContent<SetMyDefaultAdministratorRightsPostSuccess>, Error<SetMyDefaultAdministratorRightsPostError>> {
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
            let local_var_entity: Option<SetMyDefaultAdministratorRightsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetMyDefaultAdministratorRightsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
    async fn set_my_description_post<'description, 'language_code>(&self, description: Option<&'description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyDescriptionPostSuccess>, Error<SetMyDescriptionPostError>> {
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
            let local_var_entity: Option<SetMyDescriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetMyDescriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's name. Returns *True* on success.
    async fn set_my_name_post<'name, 'language_code>(&self, name: Option<&'name str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyNamePostSuccess>, Error<SetMyNamePostError>> {
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
            let local_var_entity: Option<SetMyNamePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetMyNamePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
    async fn set_my_short_description_post<'short_description, 'language_code>(&self, short_description: Option<&'short_description str>, language_code: Option<&'language_code str>) -> Result<ResponseContent<SetMyShortDescriptionPostSuccess>, Error<SetMyShortDescriptionPostError>> {
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
            let local_var_entity: Option<SetMyShortDescriptionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetMyShortDescriptionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.  Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    async fn set_passport_data_errors_post<'user_id, 'errors>(&self, user_id: i32, errors: Vec<models::PassportElementError>) -> Result<ResponseContent<SetPassportDataErrorsPostSuccess>, Error<SetPassportDataErrorsPostError>> {
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
            let local_var_entity: Option<SetPassportDataErrorsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetPassportDataErrorsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn set_sticker_emoji_list_post<'sticker, 'emoji_list>(&self, sticker: &'sticker str, emoji_list: Vec<String>) -> Result<ResponseContent<SetStickerEmojiListPostSuccess>, Error<SetStickerEmojiListPostError>> {
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
            let local_var_entity: Option<SetStickerEmojiListPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetStickerEmojiListPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    async fn set_sticker_keywords_post<'sticker, 'keywords>(&self, sticker: &'sticker str, keywords: Option<Vec<String>>) -> Result<ResponseContent<SetStickerKeywordsPostSuccess>, Error<SetStickerKeywordsPostError>> {
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
            let local_var_entity: Option<SetStickerKeywordsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetStickerKeywordsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
    async fn set_sticker_mask_position_post<'sticker, 'mask_position>(&self, sticker: &'sticker str, mask_position: Option<models::models::MaskPosition>) -> Result<ResponseContent<SetStickerMaskPositionPostSuccess>, Error<SetStickerMaskPositionPostError>> {
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
            let local_var_entity: Option<SetStickerMaskPositionPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetStickerMaskPositionPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
    async fn set_sticker_position_in_set_post<'sticker, 'position>(&self, sticker: &'sticker str, position: i32) -> Result<ResponseContent<SetStickerPositionInSetPostSuccess>, Error<SetStickerPositionInSetPostError>> {
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
            let local_var_entity: Option<SetStickerPositionInSetPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetStickerPositionInSetPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
    async fn set_sticker_set_thumbnail_post<'name, 'user_id, 'format, 'thumbnail>(&self, name: &'name str, user_id: i32, format: &'format str, thumbnail: Option<models::models::SetStickerSetThumbnailPostRequestThumbnail>) -> Result<ResponseContent<SetStickerSetThumbnailPostSuccess>, Error<SetStickerSetThumbnailPostError>> {
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
            let local_var_entity: Option<SetStickerSetThumbnailPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetStickerSetThumbnailPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to set the title of a created sticker set. Returns *True* on success.
    async fn set_sticker_set_title_post<'name, 'title>(&self, name: &'name str, title: &'title str) -> Result<ResponseContent<SetStickerSetTitlePostSuccess>, Error<SetStickerSetTitlePostError>> {
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
            let local_var_entity: Option<SetStickerSetTitlePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetStickerSetTitlePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
    async fn set_user_emoji_status_post<'user_id, 'emoji_status_custom_emoji_id, 'emoji_status_expiration_date>(&self, user_id: i32, emoji_status_custom_emoji_id: Option<&'emoji_status_custom_emoji_id str>, emoji_status_expiration_date: Option<i32>) -> Result<ResponseContent<SetUserEmojiStatusPostSuccess>, Error<SetUserEmojiStatusPostError>> {
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
            let local_var_entity: Option<SetUserEmojiStatusPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetUserEmojiStatusPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts. Returns *True* on success.  If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    async fn set_webhook_post<'url, 'certificate, 'ip_address, 'max_connections, 'allowed_updates, 'drop_pending_updates, 'secret_token>(&self, url: &'url str, certificate: Option<models::serde_json::Value>, ip_address: Option<&'ip_address str>, max_connections: Option<i32>, allowed_updates: Option<Vec<String>>, drop_pending_updates: Option<bool>, secret_token: Option<&'secret_token str>) -> Result<ResponseContent<SetWebhookPostSuccess>, Error<SetWebhookPostError>> {
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
            let local_var_entity: Option<SetWebhookPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetWebhookPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to stop updating a live location message before *live\\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    async fn stop_message_live_location_post<'business_connection_id, 'chat_id, 'message_id, 'inline_message_id, 'reply_markup>(&self, business_connection_id: Option<&'business_connection_id str>, chat_id: Option<models::models::EditMessageTextPostRequestChatId>, message_id: Option<i32>, inline_message_id: Option<&'inline_message_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<StopMessageLiveLocationPostSuccess>, Error<StopMessageLiveLocationPostError>> {
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
            let local_var_entity: Option<StopMessageLiveLocationPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<StopMessageLiveLocationPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
    async fn stop_poll_post<'chat_id, 'message_id, 'business_connection_id, 'reply_markup>(&self, chat_id: models::models::SendMessagePostRequestChatId, message_id: i32, business_connection_id: Option<&'business_connection_id str>, reply_markup: Option<models::models::InlineKeyboardMarkup>) -> Result<ResponseContent<StopPollPostSuccess>, Error<StopPollPostError>> {
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
            let local_var_entity: Option<StopPollPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<StopPollPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the *can\\_transfer\\_stars* business bot right. Returns *True* on success.
    async fn transfer_business_account_stars_post<'business_connection_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, star_count: i32) -> Result<ResponseContent<TransferBusinessAccountStarsPostSuccess>, Error<TransferBusinessAccountStarsPostError>> {
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
            let local_var_entity: Option<TransferBusinessAccountStarsPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<TransferBusinessAccountStarsPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Transfers an owned unique gift to another user. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Requires *can\\_transfer\\_stars* business bot right if the transfer is paid. Returns *True* on success.
    async fn transfer_gift_post<'business_connection_id, 'owned_gift_id, 'new_owner_chat_id, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, new_owner_chat_id: i32, star_count: Option<i32>) -> Result<ResponseContent<TransferGiftPostSuccess>, Error<TransferGiftPostError>> {
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
            let local_var_entity: Option<TransferGiftPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<TransferGiftPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\\_if\\_banned*. Returns *True* on success.
    async fn unban_chat_member_post<'chat_id, 'user_id, 'only_if_banned>(&self, chat_id: models::models::BanChatMemberPostRequestChatId, user_id: i32, only_if_banned: Option<bool>) -> Result<ResponseContent<UnbanChatMemberPostSuccess>, Error<UnbanChatMemberPostError>> {
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
            let local_var_entity: Option<UnbanChatMemberPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnbanChatMemberPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
    async fn unban_chat_sender_chat_post<'chat_id, 'sender_chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, sender_chat_id: i32) -> Result<ResponseContent<UnbanChatSenderChatPostSuccess>, Error<UnbanChatSenderChatPostError>> {
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
            let local_var_entity: Option<UnbanChatSenderChatPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnbanChatSenderChatPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\\_manage\\_topics* administrator rights. Returns *True* on success.
    async fn unhide_general_forum_topic_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<UnhideGeneralForumTopicPostSuccess>, Error<UnhideGeneralForumTopicPostError>> {
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
            let local_var_entity: Option<UnhideGeneralForumTopicPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnhideGeneralForumTopicPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn unpin_all_chat_messages_post<'chat_id>(&self, chat_id: models::models::SendMessagePostRequestChatId) -> Result<ResponseContent<UnpinAllChatMessagesPostSuccess>, Error<UnpinAllChatMessagesPostError>> {
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
            let local_var_entity: Option<UnpinAllChatMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnpinAllChatMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn unpin_all_forum_topic_messages_post<'chat_id, 'message_thread_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId, message_thread_id: i32) -> Result<ResponseContent<UnpinAllForumTopicMessagesPostSuccess>, Error<UnpinAllForumTopicMessagesPostError>> {
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
            let local_var_entity: Option<UnpinAllForumTopicMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnpinAllForumTopicMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\\_pin\\_messages* administrator right in the supergroup. Returns *True* on success.
    async fn unpin_all_general_forum_topic_messages_post<'chat_id>(&self, chat_id: models::models::RestrictChatMemberPostRequestChatId) -> Result<ResponseContent<UnpinAllGeneralForumTopicMessagesPostSuccess>, Error<UnpinAllGeneralForumTopicMessagesPostError>> {
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
            let local_var_entity: Option<UnpinAllGeneralForumTopicMessagesPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnpinAllGeneralForumTopicMessagesPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\\_pin\\_messages' administrator right in a supergroup or 'can\\_edit\\_messages' administrator right in a channel. Returns *True* on success.
    async fn unpin_chat_message_post<'chat_id, 'business_connection_id, 'message_id>(&self, chat_id: models::models::SendMessagePostRequestChatId, business_connection_id: Option<&'business_connection_id str>, message_id: Option<i32>) -> Result<ResponseContent<UnpinChatMessagePostSuccess>, Error<UnpinChatMessagePostError>> {
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
            let local_var_entity: Option<UnpinChatMessagePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnpinChatMessagePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Upgrades a given regular gift to a unique gift. Requires the *can\\_transfer\\_and\\_upgrade\\_gifts* business bot right. Additionally requires the *can\\_transfer\\_stars* business bot right if the upgrade is paid. Returns *True* on success.
    async fn upgrade_gift_post<'business_connection_id, 'owned_gift_id, 'keep_original_details, 'star_count>(&self, business_connection_id: &'business_connection_id str, owned_gift_id: &'owned_gift_id str, keep_original_details: Option<bool>, star_count: Option<i32>) -> Result<ResponseContent<UpgradeGiftPostSuccess>, Error<UpgradeGiftPostError>> {
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
            let local_var_entity: Option<UpgradeGiftPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpgradeGiftPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
    async fn upload_sticker_file_post<'user_id, 'sticker, 'sticker_format>(&self, user_id: i32, sticker: Option<models::serde_json::Value>, sticker_format: &'sticker_format str) -> Result<ResponseContent<UploadStickerFilePostSuccess>, Error<UploadStickerFilePostError>> {
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
            let local_var_entity: Option<UploadStickerFilePostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UploadStickerFilePostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn verify_chat_post<'chat_id, 'custom_description>(&self, chat_id: models::models::SendMessagePostRequestChatId, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<VerifyChatPostSuccess>, Error<VerifyChatPostError>> {
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
            let local_var_entity: Option<VerifyChatPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<VerifyChatPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
    async fn verify_user_post<'user_id, 'custom_description>(&self, user_id: i32, custom_description: Option<&'custom_description str>) -> Result<ResponseContent<VerifyUserPostSuccess>, Error<VerifyUserPostError>> {
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
            let local_var_entity: Option<VerifyUserPostSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<VerifyUserPostError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed successes of method [`add_sticker_to_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddStickerToSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`answer_callback_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerCallbackQueryPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`answer_inline_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerInlineQueryPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`answer_pre_checkout_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerPreCheckoutQueryPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`answer_shipping_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerShippingQueryPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`answer_web_app_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerWebAppQueryPostSuccess {
    Status200(models::AnswerWebAppQueryPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`approve_chat_join_request_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproveChatJoinRequestPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`ban_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanChatMemberPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`ban_chat_sender_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanChatSenderChatPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`close_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`close_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseGeneralForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`close_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClosePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`convert_gift_to_stars_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertGiftToStarsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`copy_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyMessagePostSuccess {
    Status200(models::CopyMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`copy_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyMessagesPostSuccess {
    Status200(models::ForwardMessagesPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatInviteLinkPostSuccess {
    Status200(models::CreateChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_chat_subscription_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatSubscriptionInviteLinkPostSuccess {
    Status200(models::CreateChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateForumTopicPostSuccess {
    Status200(models::CreateForumTopicPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_invoice_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInvoiceLinkPostSuccess {
    Status200(models::ExportChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_new_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewStickerSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`decline_chat_join_request_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclineChatJoinRequestPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_business_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBusinessMessagesPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_chat_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChatPhotoPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_chat_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChatStickerSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessagePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessagesPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_my_commands_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMyCommandsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_sticker_from_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStickerFromSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStickerSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_story_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStoryPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_webhook_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditChatInviteLinkPostSuccess {
    Status200(models::CreateChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_chat_subscription_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditChatSubscriptionInviteLinkPostSuccess {
    Status200(models::CreateChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditGeneralForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_message_caption_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageCaptionPostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_message_live_location_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageLiveLocationPostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_message_media_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageMediaPostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_message_reply_markup_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageReplyMarkupPostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_message_text_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageTextPostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_story_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditStoryPostSuccess {
    Status200(models::PostStoryPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_user_star_subscription_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditUserStarSubscriptionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`export_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportChatInviteLinkPostSuccess {
    Status200(models::ExportChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`forward_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardMessagePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`forward_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardMessagesPostSuccess {
    Status200(models::ForwardMessagesPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_available_gifts_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAvailableGiftsPostSuccess {
    Status200(models::GetAvailableGiftsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_business_account_gifts_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBusinessAccountGiftsPostSuccess {
    Status200(models::GetBusinessAccountGiftsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_business_account_star_balance_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBusinessAccountStarBalancePostSuccess {
    Status200(models::GetBusinessAccountStarBalancePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_business_connection_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBusinessConnectionPostSuccess {
    Status200(models::GetBusinessConnectionPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_chat_administrators_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatAdministratorsPostSuccess {
    Status200(models::GetChatAdministratorsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_chat_member_count_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatMemberCountPostSuccess {
    Status200(models::GetChatMemberCountPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatMemberPostSuccess {
    Status200(models::GetChatMemberPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_chat_menu_button_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatMenuButtonPostSuccess {
    Status200(models::GetChatMenuButtonPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatPostSuccess {
    Status200(models::GetChatPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_custom_emoji_stickers_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomEmojiStickersPostSuccess {
    Status200(models::GetForumTopicIconStickersPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_file_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilePostSuccess {
    Status200(models::GetFilePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_forum_topic_icon_stickers_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetForumTopicIconStickersPostSuccess {
    Status200(models::GetForumTopicIconStickersPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_game_high_scores_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGameHighScoresPostSuccess {
    Status200(models::GetGameHighScoresPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_me_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMePostSuccess {
    Status200(models::GetMePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_commands_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyCommandsPostSuccess {
    Status200(models::GetMyCommandsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_default_administrator_rights_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyDefaultAdministratorRightsPostSuccess {
    Status200(models::GetMyDefaultAdministratorRightsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyDescriptionPostSuccess {
    Status200(models::GetMyDescriptionPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_name_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyNamePostSuccess {
    Status200(models::GetMyNamePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_short_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyShortDescriptionPostSuccess {
    Status200(models::GetMyShortDescriptionPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_star_transactions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStarTransactionsPostSuccess {
    Status200(models::GetStarTransactionsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerSetPostSuccess {
    Status200(models::GetStickerSetPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_updates_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUpdatesPostSuccess {
    Status200(models::GetUpdatesPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_user_chat_boosts_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserChatBoostsPostSuccess {
    Status200(models::GetUserChatBoostsPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_user_profile_photos_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserProfilePhotosPostSuccess {
    Status200(models::GetUserProfilePhotosPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_webhook_info_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookInfoPostSuccess {
    Status200(models::GetWebhookInfoPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`gift_premium_subscription_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GiftPremiumSubscriptionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`hide_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HideGeneralForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`leave_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveChatPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`log_out_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogOutPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`pin_chat_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PinChatMessagePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_story_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostStoryPostSuccess {
    Status200(models::PostStoryPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`promote_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromoteChatMemberPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`read_business_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadBusinessMessagePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`refund_star_payment_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefundStarPaymentPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`remove_business_account_profile_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveBusinessAccountProfilePhotoPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`remove_chat_verification_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveChatVerificationPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`remove_user_verification_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserVerificationPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`reopen_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReopenForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`reopen_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReopenGeneralForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`replace_sticker_in_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceStickerInSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`restrict_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestrictChatMemberPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`revoke_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevokeChatInviteLinkPostSuccess {
    Status200(models::CreateChatInviteLinkPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`save_prepared_inline_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SavePreparedInlineMessagePostSuccess {
    Status200(models::SavePreparedInlineMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_animation_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendAnimationPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_audio_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendAudioPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_chat_action_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendChatActionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_contact_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendContactPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_dice_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendDicePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_document_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendDocumentPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_game_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendGamePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_gift_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendGiftPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_invoice_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInvoicePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_location_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendLocationPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_media_group_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMediaGroupPostSuccess {
    Status200(models::SendMediaGroupPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMessagePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_paid_media_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPaidMediaPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPhotoPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_poll_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPollPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_sticker_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendStickerPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_venue_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVenuePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_video_note_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVideoNotePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_video_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVideoPostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_voice_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVoicePostSuccess {
    Status200(models::SendMessagePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_business_account_bio_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountBioPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_business_account_gift_settings_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountGiftSettingsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_business_account_name_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountNamePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_business_account_profile_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountProfilePhotoPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_business_account_username_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountUsernamePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_administrator_custom_title_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatAdministratorCustomTitlePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatDescriptionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_menu_button_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatMenuButtonPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_permissions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatPermissionsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatPhotoPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatStickerSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_chat_title_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatTitlePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_custom_emoji_sticker_set_thumbnail_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCustomEmojiStickerSetThumbnailPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_game_score_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGameScorePostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_message_reaction_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMessageReactionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_my_commands_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyCommandsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_my_default_administrator_rights_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyDefaultAdministratorRightsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_my_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyDescriptionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_my_name_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyNamePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_my_short_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyShortDescriptionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_passport_data_errors_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPassportDataErrorsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_sticker_emoji_list_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerEmojiListPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_sticker_keywords_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerKeywordsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_sticker_mask_position_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerMaskPositionPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_sticker_position_in_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerPositionInSetPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_sticker_set_thumbnail_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerSetThumbnailPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_sticker_set_title_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerSetTitlePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_user_emoji_status_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetUserEmojiStatusPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_webhook_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetWebhookPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`stop_message_live_location_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopMessageLiveLocationPostSuccess {
    Status200(models::EditMessageTextPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`stop_poll_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopPollPostSuccess {
    Status200(models::StopPollPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`transfer_business_account_stars_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferBusinessAccountStarsPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`transfer_gift_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferGiftPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unban_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanChatMemberPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unban_chat_sender_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanChatSenderChatPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unhide_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnhideGeneralForumTopicPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unpin_all_chat_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinAllChatMessagesPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unpin_all_forum_topic_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinAllForumTopicMessagesPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unpin_all_general_forum_topic_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinAllGeneralForumTopicMessagesPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unpin_chat_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinChatMessagePostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`upgrade_gift_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpgradeGiftPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`upload_sticker_file_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadStickerFilePostSuccess {
    Status200(models::GetFilePost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`verify_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyChatPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`verify_user_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyUserPostSuccess {
    Status200(models::SetWebhookPost200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_sticker_to_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddStickerToSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`answer_callback_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerCallbackQueryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`answer_inline_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerInlineQueryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`answer_pre_checkout_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerPreCheckoutQueryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`answer_shipping_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerShippingQueryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`answer_web_app_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerWebAppQueryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`approve_chat_join_request_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproveChatJoinRequestPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ban_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanChatMemberPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ban_chat_sender_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanChatSenderChatPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`close_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`close_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseGeneralForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`close_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClosePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`convert_gift_to_stars_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertGiftToStarsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`copy_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`copy_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatInviteLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_chat_subscription_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatSubscriptionInviteLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_invoice_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInvoiceLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_new_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewStickerSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`decline_chat_join_request_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclineChatJoinRequestPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_business_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBusinessMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_chat_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChatPhotoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_chat_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChatStickerSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_my_commands_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMyCommandsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sticker_from_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStickerFromSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStickerSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_story_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStoryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditChatInviteLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_chat_subscription_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditChatSubscriptionInviteLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditGeneralForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_message_caption_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageCaptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_message_live_location_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageLiveLocationPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_message_media_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageMediaPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_message_reply_markup_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageReplyMarkupPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_message_text_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMessageTextPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_story_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditStoryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_user_star_subscription_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditUserStarSubscriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`export_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportChatInviteLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forward_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forward_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_available_gifts_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAvailableGiftsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_business_account_gifts_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBusinessAccountGiftsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_business_account_star_balance_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBusinessAccountStarBalancePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_business_connection_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBusinessConnectionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_chat_administrators_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatAdministratorsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_chat_member_count_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatMemberCountPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatMemberPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_chat_menu_button_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatMenuButtonPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_custom_emoji_stickers_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomEmojiStickersPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_file_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_forum_topic_icon_stickers_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetForumTopicIconStickersPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_game_high_scores_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGameHighScoresPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_me_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_commands_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyCommandsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_default_administrator_rights_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyDefaultAdministratorRightsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyDescriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_name_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyNamePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_short_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyShortDescriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_star_transactions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStarTransactionsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_updates_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUpdatesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_chat_boosts_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserChatBoostsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_profile_photos_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserProfilePhotosPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook_info_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookInfoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`gift_premium_subscription_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GiftPremiumSubscriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`hide_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HideGeneralForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveChatPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`log_out_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogOutPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pin_chat_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PinChatMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_story_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostStoryPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`promote_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromoteChatMemberPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_business_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadBusinessMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`refund_star_payment_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefundStarPaymentPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_business_account_profile_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveBusinessAccountProfilePhotoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_chat_verification_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveChatVerificationPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_user_verification_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserVerificationPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reopen_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReopenForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reopen_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReopenGeneralForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_sticker_in_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceStickerInSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`restrict_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestrictChatMemberPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`revoke_chat_invite_link_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevokeChatInviteLinkPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`save_prepared_inline_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SavePreparedInlineMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_animation_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendAnimationPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_audio_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendAudioPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_chat_action_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendChatActionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_contact_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendContactPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_dice_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendDicePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_document_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendDocumentPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_game_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendGamePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_gift_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendGiftPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_invoice_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInvoicePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_location_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendLocationPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_media_group_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMediaGroupPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_paid_media_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPaidMediaPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPhotoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_poll_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPollPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_sticker_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendStickerPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_venue_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVenuePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_video_note_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVideoNotePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_video_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVideoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_voice_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendVoicePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_business_account_bio_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountBioPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_business_account_gift_settings_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountGiftSettingsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_business_account_name_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountNamePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_business_account_profile_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountProfilePhotoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_business_account_username_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetBusinessAccountUsernamePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_administrator_custom_title_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatAdministratorCustomTitlePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatDescriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_menu_button_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatMenuButtonPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_permissions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatPermissionsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_photo_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatPhotoPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_sticker_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatStickerSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_chat_title_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChatTitlePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_custom_emoji_sticker_set_thumbnail_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCustomEmojiStickerSetThumbnailPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_game_score_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGameScorePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_message_reaction_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMessageReactionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_my_commands_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyCommandsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_my_default_administrator_rights_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyDefaultAdministratorRightsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_my_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyDescriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_my_name_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyNamePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_my_short_description_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetMyShortDescriptionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_passport_data_errors_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPassportDataErrorsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_sticker_emoji_list_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerEmojiListPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_sticker_keywords_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerKeywordsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_sticker_mask_position_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerMaskPositionPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_sticker_position_in_set_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerPositionInSetPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_sticker_set_thumbnail_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerSetThumbnailPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_sticker_set_title_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetStickerSetTitlePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_user_emoji_status_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetUserEmojiStatusPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_webhook_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetWebhookPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stop_message_live_location_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopMessageLiveLocationPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stop_poll_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopPollPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transfer_business_account_stars_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferBusinessAccountStarsPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transfer_gift_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferGiftPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unban_chat_member_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanChatMemberPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unban_chat_sender_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanChatSenderChatPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unhide_general_forum_topic_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnhideGeneralForumTopicPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unpin_all_chat_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinAllChatMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unpin_all_forum_topic_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinAllForumTopicMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unpin_all_general_forum_topic_messages_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinAllGeneralForumTopicMessagesPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unpin_chat_message_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinChatMessagePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upgrade_gift_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpgradeGiftPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_sticker_file_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadStickerFilePostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_chat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyChatPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_user_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyUserPostError {
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

