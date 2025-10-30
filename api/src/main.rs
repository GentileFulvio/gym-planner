use axum::{Router};
use std::sync::{Arc};
use connectors::{club_connector_default::DefaultClubConnector, member_connector_default::MemberConnector};
use application::{club_management_application::ClubManagementApplication, member_management_application::MemberManagementApplication};
use crate::routes::{clubs, members};

mod routes;
mod dto;

#[derive(Clone)]
pub struct AppState {
    pub member_service: Arc<MemberManagementApplication<MemberConnector>>,
    pub club_service: Arc<ClubManagementApplication<DefaultClubConnector>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let member_repo = MemberConnector::new();
    let member_service = Arc::new(MemberManagementApplication::new(Box::new(member_repo)));
    let club_repo = DefaultClubConnector::new();
    let club_service = Arc::new(ClubManagementApplication::new(Box::new(club_repo)));

    let state = AppState { member_service, club_service };

    let app = Router::new()
        .nest("/members", members::router())
        .nest("/clubs", clubs::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await?;

    Ok(())
}
