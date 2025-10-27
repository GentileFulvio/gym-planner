use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct MemberRecord {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub struct CreateMemberDto {
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait MemberConnectorTrait: Send + Sync {
    async fn get_by_id(&self, id: String) -> anyhow::Result<Option<MemberRecord>>;
    async fn save(&self, member: &CreateMemberDto) -> anyhow::Result<MemberRecord>;
    async fn get_all(&self) -> anyhow::Result<Vec<MemberRecord>>;
}
