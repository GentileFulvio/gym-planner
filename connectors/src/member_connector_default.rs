use std::{collections::HashMap, sync::{Arc, RwLock}};

use anyhow::Ok;
use async_trait::async_trait;
use crate::member_connector_trait::{MemberConnectorTrait, MemberRecord, CreateMemberDto};

pub struct MemberConnector {
    data: Arc<RwLock<HashMap<String, MemberRecord>>>,
}

impl MemberConnector {
    pub fn new() -> Self {
        Self {
            /*
             * Arc and RwLock are used here to allow safe concurrent access to the in-memory data store.
             * If we don't do this we need to make all methods &mut self which would complicate especially since the actual implementation
             * does not need mutability this is solely for the in-memory mock.
             */
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MemberConnectorTrait for MemberConnector {
    async fn get_by_id(&self, id: String) -> anyhow::Result<Option<MemberRecord>> {
        let map = self.data.read().unwrap();
        let record = map.get(&id);

        Ok(record.cloned())
    }

    async fn save(&self, member: &CreateMemberDto) -> anyhow::Result<MemberRecord> {
        // Create date string for now as id\
        let mut map = self.data.write().unwrap();
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let record = MemberRecord {
            id,
            name: member.name.clone(),
            email: member.email.clone(),
        };

        map.insert(record.id.clone(), record.clone());

        Ok(record)
    }

    async fn get_all(&self) -> anyhow::Result<Vec<MemberRecord>> {
        let map = self.data.read().unwrap();
        Ok(map.values().cloned().collect())
    }
}