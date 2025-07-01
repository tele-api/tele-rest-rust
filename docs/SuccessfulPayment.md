# SuccessfulPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **String** | Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90) | 
**total_amount** | **i32** | Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). | 
**invoice_payload** | **String** | Bot-specified invoice payload | 
**subscription_expiration_date** | Option<**i32**> | *Optional*. Expiration date of the subscription, in Unix time; for recurring payments only | [optional]
**is_recurring** | Option<**bool**> | *Optional*. True, if the payment is a recurring payment for a subscription | [optional][default to true]
**is_first_recurring** | Option<**bool**> | *Optional*. True, if the payment is the first payment for a subscription | [optional][default to true]
**shipping_option_id** | Option<**String**> | *Optional*. Identifier of the shipping option chosen by the user | [optional]
**order_info** | Option<[**models::OrderInfo**](OrderInfo.md)> |  | [optional]
**telegram_payment_charge_id** | **String** | Telegram payment identifier | 
**provider_payment_charge_id** | **String** | Provider payment identifier | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


