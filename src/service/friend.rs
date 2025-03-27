#![allow(unused)]

use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use serde::Serialize;
use crate::repository::friends::FriendsRepository;
use super::wss::WssService;

pub struct FriendService;

impl FriendService {
  pub fn send_event<T: Serialize + Clone>(
    db: &mut Database<Postgres>,
    user_id: i32,
    event_name: &'static str,
    payload: T
  ) -> NonJsonHttpResult<()> {
    let friends = FriendsRepository::get_friend_ids(db, user_id)
      .map_err(|_| HttpError::new("Не получилось получить список друзей пользователя", None))?;

    for friend in friends {
      WssService::send(friend, event_name, payload.clone())?;
    }

    Ok(())
  }
}