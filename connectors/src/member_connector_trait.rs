use async_trait::async_trait;

pub struct MemberRecord {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait MemberRepository: Send + Sync {
    async fn get_by_id(&self, id: String) -> anyhow::Result<Option<MemberRecord>>;
    async fn save(&self, member: &MemberRecord) -> anyhow::Result<()>;
}
