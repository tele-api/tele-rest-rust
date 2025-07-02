# SetWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | HTTPS URL to send updates to. Use an empty string to remove webhook integration | 
**certificate** | Option<[**serde_json::Value**](.md)> |  | [optional]
**ip_address** | Option<**String**> | The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS | [optional]
**max_connections** | Option<**i32**> | The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to *40*. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput. | [optional][default to 40]
**allowed_updates** | Option<**Vec<String>**> | A JSON-serialized list of the update types you want your bot to receive. For example, specify `[\"message\", \"edited_channel_post\", \"callback_query\"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\\_member*, *message\\_reaction*, and *message\\_reaction\\_count* (default). If not specified, the previous setting will be used.   Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time. | [optional]
**drop_pending_updates** | Option<**bool**> | Pass *True* to drop all pending updates | [optional]
**secret_token** | Option<**String**> | A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure that the request comes from a webhook set by you. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


