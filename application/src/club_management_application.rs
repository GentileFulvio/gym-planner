use connectors::club_connector_trait::ClubConnectorTrait;
use domain::club::Club;

pub struct ClubManagementApplication<R: ClubConnectorTrait> {
    repository: Box<R>,
}

impl<R: ClubConnectorTrait> ClubManagementApplication<R> {
    pub fn new(repository: Box<R>) -> Self {
        Self { repository }
    }

    pub async fn list_all_clubs(&self) -> anyhow::Result<Vec<Club>> {
        let club_entity_list = self.repository.get_all_clubs().await?;

        Ok(club_entity_list.iter().map(|data| {
            Club {
                id: data.id.clone(),
                name: data.name.clone(),
                registration_date: data.registration_date.clone(),
            }
        }).collect())
    }

    pub async fn find_club_by_id(&self, id: String) -> anyhow::Result<Option<Club>> {
        let result = self.repository.get_club_by_id(id).await?;

        Ok(result.map(|data| {
            Club {
                id: data.id,
                name: data.name,
                registration_date: data.registration_date,
            }
        }))
    }

    pub async fn register_club(&self, name: String) -> anyhow::Result<Club> {
        let club_dto = connectors::club_connector_trait::CreateClubDto {
            name,
           registration_date: "TODO".to_string()
        };

        let club_record = self.repository.add_club(club_dto).await?;

        Ok(Club {
            id: club_record.id,
            name: club_record.name,
            registration_date: club_record.registration_date,
        })
    }
}