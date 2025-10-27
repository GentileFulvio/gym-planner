use connectors::member_connector_trait::{CreateMemberDto, MemberConnectorTrait};
use domain::member::Member;

pub struct MemberManagementApplication<R: MemberConnectorTrait> {
    repository: Box<R>,
}

impl<R: MemberConnectorTrait> MemberManagementApplication<R> {
    pub fn new(repository: Box<R>) -> Self {
        Self { repository }
    }

    pub async fn register_member(&self, name: String, email: String) -> anyhow::Result<Member> {
        let member = CreateMemberDto { name, email };

        self.repository.save(&member).await.map(|data| {
            Member::new(data.id, data.name, data.email)
        })
    }

    pub async fn find_member_by_id(&self, id: String) -> anyhow::Result<Option<Member>> {
        let result = self.repository.get_by_id(id).await?;

        Ok(result.map(|data| Member::new(data.id, data.name, data.email)))
    }
}
