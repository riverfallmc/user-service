use adjust::{database::{redis::Redis, Database}, redis::Commands, response::{HttpError, NonJsonHttpResult}};
use reqwest::StatusCode;

const INVITE_COOLDOWN: u64 = 5;

pub struct InviteService;

impl InviteService {
  fn get_key(inviter: i32, invited: i32) -> String {
    format!("invitecd:{inviter}:{invited}")
  }

  pub fn check_send(redis: &mut Database<Redis>, inviter: i32, invited: i32) -> NonJsonHttpResult<()> {
    let key = Self::get_key(inviter, invited);

    if redis.get::<_, Option<bool>>(&key)?.is_some() {
      return Err(HttpError::new("Не так быстро, ковбой!", Some(StatusCode::TOO_MANY_REQUESTS)));
    }

    redis.set_ex::<_, _, ()>(key, true, INVITE_COOLDOWN)?;

    Ok(())
  }
}