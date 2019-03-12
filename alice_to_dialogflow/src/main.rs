use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use config::{Config, Environment};
use log::error;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, watcher, Watcher};
use serde::Deserialize;

mod dialogs;
mod dialogs_dialogflow_handler;
mod dialogflow;


#[derive(Deserialize, Debug)]
pub struct Settings {
    pub project_id: String,
    pub authorized_dialog_ids_path: String,
    pub dialogflow_proxy_uri: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Self {
        let mut config = Config::default();
        config.merge(Environment::new()).expect("Failed merge env config");

        config.try_into().expect("Failed convert to config")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthorizedDialogs {
    pub ids: Option<Vec<String>>
}


impl AuthorizedDialogs {
    pub fn new_hot<S: Into<String>>(path: S) -> impl Fn() -> AuthorizedDialogs {
        let path = path.into();
        let mut config = Config::default();
        config.merge(config::File::new(&path, config::FileFormat::Json).required(true)).expect("Failed to merge json file with config");

        let settings_cache = Arc::new(RwLock::new(config.clone().try_into::<AuthorizedDialogs>().expect("Failed to convert config")));

        let in_updater_cachhe = settings_cache.clone();
        let updater_thread = thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher: RecommendedWatcher = watcher(tx, Duration::from_secs(2)).expect("Failed to create watcher on config");
            watcher
                .watch(path, RecursiveMode::NonRecursive)
                .expect("Failed to start watcher on config");

            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::Write(_)) => {
                        match config.refresh() {
                            Err(e) => error!("refresh error: {:?}", e),
                            Ok(new_conf) => {
                                match new_conf.clone().try_into::<AuthorizedDialogs>() {
                                    Ok(settings) => {
                                        *in_updater_cachhe.write().unwrap() = settings;
                                    }
                                    Err(e) => {
                                        error!("Failed to convert config {:?}", e);
                                    }
                                }
                            }
                        }
                    }

                    Err(e) => {
                        error!("watch error: {:?}", e);
                        return;
                    }

                    _ => {}
                }
            }
        });

        move || {
            &updater_thread;
            settings_cache.read().unwrap().clone()
        }
    }
}


fn main() {
    env_logger::init();
    let settings = Settings::new();
    let authorized_dialogs = AuthorizedDialogs::new_hot(settings.authorized_dialog_ids_path);
    let dialogflow_client = dialogflow::DialogflowClient::new(settings.dialogflow_proxy_uri);

    let get_user_ids = move || authorized_dialogs().ids.unwrap_or(Vec::new());
    let dialogs_dialogflow_handler = dialogs_dialogflow_handler::DialogsDialogflowHandler::new(dialogflow_client, settings.project_id, get_user_ids);

    dialogs::listen_requests("dtd", settings.port, dialogs_dialogflow_handler);
}
