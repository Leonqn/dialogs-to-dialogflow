use hyper::{Client, Body, StatusCode};
use hyper::client::HttpConnector;
use crate::dialogflow::request::Request;
use crate::dialogflow::response::{Response};
use core::fmt;
use futures::Future;
use futures::stream::Stream;
use std::str;

pub mod request;
pub mod response;

#[derive(Debug)]
pub enum Error {
    HyperError(hyper::Error),
    BadResponse(StatusCode, String),
    SerdeError(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::HyperError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err)
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HyperError(hyper) =>
                write!(f, "Http error: {}", hyper),
            Error::BadResponse(code, message) =>
                write!(f, "Bad response. Code: {}, message: {}", code, message),
            Error::SerdeError(serde) =>
                write!(f, "Serde error: {}", serde),
        }
    }
}

pub struct DialogflowClient {
    http_client: Client<HttpConnector, Body>,
    proxy_uri: String,
}

impl DialogflowClient {
    pub fn new<S: Into<String>>(proxy_uri: S) -> Self {
        Self {
            http_client: hyper::Client::new(),
            proxy_uri: proxy_uri.into(),
        }
    }

    pub fn detect_intent(&self, request: Request) -> impl Future<Item=Response, Error=Error> {
        let http_request =
            hyper::Request::post(&self.proxy_uri)
                .body(Body::from(serde_json::to_string(&request).expect("Error occurred while serializing")))
                .expect("While creating request an error has occurred");

        self.http_client.request(http_request)
            .and_then(|r| {
                let status = r.status();
                r.into_body().concat2().map(move |body| (status, body))
            })
            .then(|result| {
                let (status, body) = result?;

                if status.is_success() {
                    Ok(serde_json::from_slice::<Response>(&body)?)
                } else {
                    let string_body = str::from_utf8(&body)
                        .map_err(|_| Error::BadResponse(status, "utf8 error".to_string()))?.to_string();
                    Err(Error::BadResponse(status, string_body))
                }
            })
    }
}