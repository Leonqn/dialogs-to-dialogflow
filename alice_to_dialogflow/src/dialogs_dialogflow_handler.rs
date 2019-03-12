use std::sync::Arc;

use futures::Future;
use log::{error, info};

use crate::dialogflow::DialogflowClient;
use crate::dialogflow::request;
use crate::dialogs;
use crate::dialogs::DialogsRequestsHandler;
use crate::dialogs::web_hook_request::WebHookRequest;
use crate::dialogs::web_hook_response::{Session, WebHookResponse};

pub struct DialogsDialogflowHandler<F> {
    inner: Arc<Inner<F>>
}

//deriving not working due to https://github.com/rust-lang/rust/issues/26925
impl<F> Clone for DialogsDialogflowHandler<F> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

struct Inner<F> {
    dialogflow_client: DialogflowClient,
    project_id: String,
    get_allowed_userids: F,
}

impl<F> DialogsDialogflowHandler<F> {
    pub fn new<S: Into<String>>(dialogflow_client: DialogflowClient, project_id: S, get_allowed_userids: F) -> Self {
        Self {
            inner: Arc::from(Inner {
                dialogflow_client,
                project_id: project_id.into(),
                get_allowed_userids,
            })
        }
    }
}

impl<G: Fn() -> Vec<String>> DialogsRequestsHandler for DialogsDialogflowHandler<G> {
    type F = Box<Future<Item=WebHookResponse, Error=()> + Send>;

    fn on_request(&self, request: WebHookRequest) -> Self::F {
        let session_id = request.session.session_id.clone();
        let original_utterance = request.request.original_utterance;
        let version = request.version;
        let response_session = Session {
            session_id: request.session.session_id,
            message_id: request.session.message_id,
            user_id: request.session.user_id,
        };

        if (self.inner.get_allowed_userids)().contains(&response_session.user_id) {
            Box::new(self.inner.dialogflow_client.detect_intent(request::Request {
                project_id: self.inner.project_id.clone(),
                session_id,
                text: original_utterance,
                language_code: "ru-RU".to_string(),
            })
                .map(move |resp| {
                    dialogs::web_hook_response::WebHookResponse {
                        response: dialogs::web_hook_response::Response {
                            text: resp.query_result.fulfillment_text.clone(),
                            tts: resp.query_result.fulfillment_text,
                            end_session: resp.query_result.diagnostic_info.map(|x| x.end_conversation).unwrap_or(true),
                        },
                        session: response_session,
                        version,
                    }
                }).map_err(|x| error!("Error occurred while detecting intent {:?}", x))
            )
        } else {
            info!("User with id {} trying to send command", &response_session.user_id);
            Box::new(futures::finished(dialogs::web_hook_response::WebHookResponse {
                response: dialogs::web_hook_response::Response {
                    text: "Это приватный навык. Вы не авторизованы для работы с этим навыком".to_string(),
                    tts: "Это приватный навык. Вы не авторизованы для работы с этим навыком".to_string(),
                    end_session: true,
                },
                session: response_session,
                version,
            }))
        }
    }
}

