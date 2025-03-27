use std::sync::Arc;
use adjust::{controller::Controller, controllers, database::{postgres::Postgres, redis::Redis, Pool}, main, service::Service};
use controller::{friends::FriendsController, invite::InviteController, privacy::PrivacyController, status::StatusController, user::UserController};

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[derive(Default, Clone)]
struct AppState {
  postgres: Arc<Pool<Postgres>>,
  redis: Pool<Redis>
}

#[main]
async fn main() -> Service<'_, AppState> {
  adjust::server::WebServer::enviroment();

  Service {
    name: "User",
    state: AppState::default(),
    controllers: controllers![UserController, InviteController, FriendsController, PrivacyController, StatusController],
    ..Default::default()
  }
}