use serde::{Deserialize, Serialize};
use domain::member::Member;

#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl MemberResponse {
    pub fn from_domain(m: &Member) -> Self {
        Self {
            id: m.id.to_string(),
            name: m.name.clone(),
            email: m.email.clone(),
        }
    }
}
