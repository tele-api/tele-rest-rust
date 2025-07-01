# InputMessageContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_text** | **String** | Text of the message to be sent, 1-4096 characters | 
**parse_mode** | Option<**String**> | *Optional*. Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. | [optional]
**entities** | Option<[**Vec<models::MessageEntity>**](MessageEntity.md)> | *Optional*. List of special entities that appear in message text, which can be specified instead of *parse\\_mode* | [optional]
**link_preview_options** | Option<[**models::LinkPreviewOptions**](LinkPreviewOptions.md)> |  | [optional]
**latitude** | **f64** | Latitude of the venue in degrees | 
**longitude** | **f64** | Longitude of the venue in degrees | 
**horizontal_accuracy** | Option<**f64**> | *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500 | [optional]
**live_period** | Option<**i32**> | *Optional*. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely. | [optional]
**heading** | Option<**i32**> | *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified. | [optional]
**proximity_alert_radius** | Option<**i32**> | *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified. | [optional]
**title** | **String** | Product name, 1-32 characters | 
**address** | **String** | Address of the venue | 
**foursquare_id** | Option<**String**> | *Optional*. Foursquare identifier of the venue, if known | [optional]
**foursquare_type** | Option<**String**> | *Optional*. Foursquare type of the venue, if known. (For example, “arts\\_entertainment/default”, “arts\\_entertainment/aquarium” or “food/icecream”.) | [optional]
**google_place_id** | Option<**String**> | *Optional*. Google Places identifier of the venue | [optional]
**google_place_type** | Option<**String**> | *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).) | [optional]
**phone_number** | **String** | Contact's phone number | 
**first_name** | **String** | Contact's first name | 
**last_name** | Option<**String**> | *Optional*. Contact's last name | [optional]
**vcard** | Option<**String**> | *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes | [optional]
**description** | **String** | Product description, 1-255 characters | 
**payload** | **String** | Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes. | 
**provider_token** | Option<**String**> | *Optional*. Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**currency** | **String** | Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90). | 
**prices** | [**Vec<models::LabeledPrice>**](LabeledPrice.md) | Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90). | 
**max_tip_amount** | Option<**i32**> | *Optional*. The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional][default to 0]
**suggested_tip_amounts** | Option<**Vec<i32>**> | *Optional*. A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\\_tip\\_amount*. | [optional]
**provider_data** | Option<**String**> | *Optional*. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider. | [optional]
**photo_url** | Option<**String**> | *Optional*. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. | [optional]
**photo_size** | Option<**i32**> | *Optional*. Photo size in bytes | [optional]
**photo_width** | Option<**i32**> | *Optional*. Photo width | [optional]
**photo_height** | Option<**i32**> | *Optional*. Photo height | [optional]
**need_name** | Option<**bool**> | *Optional*. Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**need_phone_number** | Option<**bool**> | *Optional*. Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**need_email** | Option<**bool**> | *Optional*. Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**need_shipping_address** | Option<**bool**> | *Optional*. Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**send_phone_number_to_provider** | Option<**bool**> | *Optional*. Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**send_email_to_provider** | Option<**bool**> | *Optional*. Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]
**is_flexible** | Option<**bool**> | *Optional*. Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


