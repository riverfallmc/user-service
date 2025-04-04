#![allow(unused)]

use adjust::{load_env, response::NonJsonHttpResult};
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::sync::LazyLock;

load_env!(WSS_URL);

static CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

pub struct WssService;

impl WssService {
  pub fn send<T: Serialize>(user_id: i32, event_name: &'static str, payload: T) -> NonJsonHttpResult<()> {
    let payload = serde_json::to_string(&payload)?;

    tokio::spawn(async move {
      CLIENT.post(format!("http://{}/send/{user_id}?type={event_name}", *WSS_URL))
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await;
    });

    Ok(())
  }

  pub fn broadcast<T: Serialize>(user_ids: Vec<i32>, event_name: String, payload: T) -> NonJsonHttpResult<()> {
    let payload = serde_json::to_string(&payload)?;

    tokio::spawn(async move {
      CLIENT.post(format!("http://{}/broadcast?type={event_name}", *WSS_URL))
        .header("Content-Type", "application/json")
        .body(json!({
          "ids": user_ids,
          "body": payload
        }).to_string())
        .send()
        .await;
    });

    Ok(())
  }
}