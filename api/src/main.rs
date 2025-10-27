use axum::{Router};
use std::sync::{Arc};
use connectors::{member_connector_default::MemberConnector};
use application::member_management_application::MemberManagementApplication;
use crate::routes::members;

mod routes;
mod dto;

#[derive(Clone)]
pub struct AppState {
    pub member_service: Arc<MemberManagementApplication<MemberConnector>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let member_repo = MemberConnector::new();
    let member_service = Arc::new(MemberManagementApplication::new(Box::new(member_repo)));

    let state = AppState { member_service };

    let app = Router::new()
        .nest("/members", members::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await?;

    Ok(())
}
