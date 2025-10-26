use connectors::member_connector_trait::{MemberRecord, MemberRepository};

pub struct MemberManagementApplication<R: MemberRepository> {
    repository: Box<R>,
}

impl<R: MemberRepository> MemberManagementApplication<R> {
    pub fn new(repository: Box<R>) -> Self {
        Self { repository }
    }

    pub async fn register_member(&self, id: String, name: String, email: String) -> anyhow::Result<()> {
        let member = MemberRecord { id, name, email };
        self.repository.save(&member).await
    }
}
