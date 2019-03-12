use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub project_id: String,
    pub session_id: String,
    pub text: String,
    pub event: String,
    pub language_code: String,
}