# StarTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with *SuccessfulPayment.telegram\\_payment\\_charge\\_id* for successful incoming payments from users. | 
**amount** | **i32** | Integer amount of Telegram Stars transferred by the transaction | 
**nanostar_amount** | Option<**i32**> | *Optional*. The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999 | [optional]
**date** | **i32** | Date the transaction was created in Unix time | 
**source** | Option<[**models::TransactionPartner**](TransactionPartner.md)> |  | [optional]
**receiver** | Option<[**models::TransactionPartner**](TransactionPartner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


