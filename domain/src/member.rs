pub struct Member {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Member {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}