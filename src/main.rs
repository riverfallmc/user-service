use dixxxie::{
  axum::{self, Router}, connection::establish_connection, controller::ApplyControllerOnRouter, setup
};
use anyhow::Result;
use controller::user::UserController;

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
  setup()?;

  let router = Router::new()
    .apply_controller(UserController)
    .with_state(establish_connection()?);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
    .await?;

  Ok(axum::serve(listener, router).await?)
}