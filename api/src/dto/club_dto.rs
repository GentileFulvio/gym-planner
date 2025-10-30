
#[derive(serde::Deserialize)]
pub struct CreateClubRequest {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct ClubResponse {
    pub id: String,
    pub name: String,
}

impl ClubResponse {
    pub fn from_domain(club: &domain::club::Club) -> Self {
        Self {
            id: club.id.clone(),
            name: club.name.clone(),
        }
    }
}