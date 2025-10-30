use std::{collections::HashMap, sync::{Arc, RwLock}};

use async_trait::async_trait;

use crate::club_connector_trait::{Club, ClubConnectorTrait, CreateClubDto};

pub struct DefaultClubConnector {
    clubs: Arc<RwLock<HashMap<String, Club>>>,
}

impl DefaultClubConnector {
    pub fn new() -> Self {
        Self {
            clubs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ClubConnectorTrait for DefaultClubConnector {
    async fn get_all_clubs(&self) -> anyhow::Result<Vec<Club>> {
        let data = self.clubs.read().unwrap();

        Ok(data.values().cloned().collect())
    }

    async fn add_club(&self, club: CreateClubDto) -> anyhow::Result<Club> {
        let mut data = self.clubs.write().unwrap();
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let new_club = Club {
            id: id.clone(),
            name: club.name,
            registration_date: club.registration_date,
        };

        data.insert(id.clone(), new_club.clone());

        Ok(new_club)
    }

    async fn get_club_by_id(&self, id: String) -> anyhow::Result<Option<Club>> {
        let data = self.clubs.read().unwrap();
        Ok(data.get(&id).cloned())
    }
}