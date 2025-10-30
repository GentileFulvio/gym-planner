use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Club {
    pub id: String,
    pub name: String,
    pub registration_date: String,
}

pub struct CreateClubDto {
    pub name: String,
    pub registration_date: String,
}

#[async_trait]
pub trait ClubConnectorTrait: Send + Sync {
    async fn get_all_clubs(&self) -> anyhow::Result<Vec<Club>>;
    async fn add_club(&self, club: CreateClubDto) -> anyhow::Result<Club>;
    async fn get_club_by_id(&self, id: String) -> anyhow::Result<Option<Club>>;
}