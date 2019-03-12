use futures::Future;
use warp;
use warp::Filter;

use crate::dialogs::web_hook_request::WebHookRequest;
use crate::dialogs::web_hook_response::WebHookResponse;

pub mod web_hook_request;
pub mod web_hook_response;


pub trait DialogsRequestsHandler {
    type F: Future<Item=WebHookResponse, Error=()> + Send;

    fn on_request(&self, request: WebHookRequest) -> Self::F;
}

pub fn listen_requests<H: DialogsRequestsHandler + Send + Sync + Clone + 'static>(prefix: &'static str, port: u16, handler: H) {
    let promote = warp::path(prefix)
        .and(warp::body::json())
        .and_then(move |body: WebHookRequest| {
            handler
                .on_request(body)
                .map_err(|_| warp::reject::custom("Unknown error occurred"))
        })
        .map(|resp| warp::reply::json(&resp));

    warp::serve(promote)
        .run(([0, 0, 0, 0], port));
}