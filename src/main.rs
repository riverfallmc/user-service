use std::sync::Arc;
use adjust::{controllers, controller::Controller, database::{postgres::Postgres, Pool}, main, service::Service};
use controller::user::UserController;

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[derive(Default, Clone)]
struct AppState {
  postgres: Arc<Pool<Postgres>>
}


#[main]
async fn main() -> Service<'_, AppState> {
  Service {
    name: "User",
    state: AppState::default(),
    controllers: controllers![UserController],
    ..Default::default()
  }
}