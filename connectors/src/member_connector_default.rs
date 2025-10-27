use anyhow::Ok;
use async_trait::async_trait;
use crate::member_connector_trait::{MemberConnectorTrait, MemberRecord, CreateMemberDto};

pub struct MemberConnector {

}

impl MemberConnector {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl MemberConnectorTrait for MemberConnector {
    async fn get_by_id(&self, id: String) -> anyhow::Result<Option<MemberRecord>> {
        Ok(Some(MemberRecord {
            id: id.clone(),
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        }))
    }

    async fn save(&self, member: &CreateMemberDto) -> anyhow::Result<MemberRecord> {
        println!("Saving member: {} - {}", member.name, member.email);
        Ok(MemberRecord {
            id: "generated-id-123".to_string(),
            name: member.name.clone(),
            email: member.email.clone(),
        })
    }
}