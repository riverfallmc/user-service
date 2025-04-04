#![allow(unused)]
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::{models::privacy::Visibility, repository::{friends::FriendsRepository, privacy::PrivacyRepository}};
use super::wss::WssService;

#[derive(Deserialize, Serialize)]
pub enum FriendEvents {
  #[serde(rename="FRIEND_ONLINE")]
  FriendOnline,
  #[serde(rename="FRIEND_OFFLINE")]
  FriendOffline,
  #[serde(rename="FRIEND_JOIN_SERVER")]
  FriendJoinServer,
  #[serde(rename="FRIEND_DISCONNECT")]
  FriendDisconnect,
  #[serde(rename="ANY")]
  Any
}

pub struct FriendService;

impl FriendService {
  pub fn send_event<T: Serialize + Clone>(
    db: &mut Database<Postgres>,
    user_id: i32,
    event: FriendEvents,
    payload: T
  ) -> NonJsonHttpResult<()> {
    let privacy_settings = PrivacyRepository::get_settings(db, user_id)?;

    // проверяем, можно ли отправлять ивент
    // согласно настройкам пользователя
    let can_see = match event {
      FriendEvents::FriendOnline | FriendEvents::FriendOffline => privacy_settings.online_visibility != Visibility::Hidden,
      FriendEvents::FriendJoinServer | FriendEvents::FriendDisconnect => privacy_settings.server_visibility != Visibility::Hidden,
      FriendEvents::Any => true
    };

    if !can_see {
      // отправляем пустое сообщение... ну типо я никак не могу убрать строку,
      // так что она будет пустой
      // ибо тут никакого смысла что-то писать нет - эндпоинт внутренний, и используется
      // только микросервисами, которым похуй на то кто и что вернул в теле
      return Err(HttpError::new("", Some(StatusCode::FORBIDDEN)));
    }

    let friends = FriendsRepository::get_friend_ids(db, user_id)
      .map_err(|_| HttpError::new("", Some(StatusCode::CONFLICT)))?; // нет друзей

    WssService::broadcast(friends, serde_json::to_string(&event)?.replace('"', ""), payload)?;

    Ok(())
  }
}