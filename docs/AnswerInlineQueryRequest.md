# AnswerInlineQueryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inline_query_id** | **String** | Unique identifier for the answered query | 
**results** | [**Vec<models::InlineQueryResult>**](InlineQueryResult.md) | A JSON-serialized array of results for the inline query | 
**cache_time** | Option<**i32**> | The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300. | [optional][default to 300]
**is_personal** | Option<**bool**> | Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query. | [optional]
**next_offset** | Option<**String**> | Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes. | [optional]
**button** | Option<[**models::InlineQueryResultsButton**](InlineQueryResultsButton.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


