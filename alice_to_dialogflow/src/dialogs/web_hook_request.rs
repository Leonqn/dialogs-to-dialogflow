use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Nlu {
    pub tokens: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Request {
    pub nlu: Nlu,
    pub original_utterance: String,
}

#[derive(Deserialize, Debug)]
pub struct WebHookRequest {
    pub request: Request,
    pub session: Session,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Session {
    pub new: bool,
    pub message_id: i64,
    pub session_id: String,
    pub skill_id: String,
    pub user_id: String,
}

