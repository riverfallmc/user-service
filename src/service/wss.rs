#![allow(unused)]

use adjust::{load_env, response::NonJsonHttpResult};
use reqwest::Client;
use serde::Serialize;
use std::sync::LazyLock;

load_env!(WSS_URL);

static CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

pub struct WssService;

impl WssService {
  pub fn send<T: Serialize>(user_id: i32, event_name: &'static str, payload: T) -> NonJsonHttpResult<()> {
    let payload = serde_json::to_string(&payload)?;

    tokio::spawn(async move {
      let res = CLIENT.post(format!("http://{}/send/{user_id}?type={event_name}", *WSS_URL))
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await;
    });

    Ok(())
  }
}