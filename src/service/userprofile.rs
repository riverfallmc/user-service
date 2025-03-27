use adjust::{database::{postgres::Postgres, redis::Redis, Database}, response::NonJsonHttpResult};
use crate::{models::{user::UserIdQuery, userprofile::UserProfile}, repository::user::UserRepository};
use super::{privacy::PrivacyService, status::StatusService};

pub struct UserProfileService;

impl UserProfileService {
  pub fn get_user_profile(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    query: Option<UserIdQuery>,
    user_id: i32
  ) -> NonJsonHttpResult<UserProfile> {
    PrivacyService::check_profile_visibility(db, query.unwrap_or(UserIdQuery { user_id: Some(user_id) }), user_id)?;

    let user = UserRepository::get_user(db, user_id)?;

    Ok(UserProfile {
      id: user.id,
      username: user.username,
      rank: user.rank,
      status: StatusService::get_full_status(db, redis, user_id, Some(user.last_seen_at))?,
      registered_at: user.registered_at,
    })
  }
}