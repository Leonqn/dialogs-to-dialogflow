use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Response {
    pub text: String,
    pub tts: String,
    pub end_session: bool,
}

#[derive(Serialize, Debug)]
pub struct WebHookResponse {
    pub response: Response,
    pub session: Session,
    pub version: String,
}

#[derive(Serialize, Debug)]
pub struct Session {
    pub session_id: String,
    pub message_id: i64,
    pub user_id: String,
}

