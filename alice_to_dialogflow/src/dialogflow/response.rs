use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub response_id: String,
    pub query_result: QueryResult,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub fulfillment_text: String,
    pub diagnostic_info: Option<DiagnosticInfo>
}

#[derive(Deserialize, Debug)]
pub struct DiagnosticInfo {
    pub end_conversation: bool
}