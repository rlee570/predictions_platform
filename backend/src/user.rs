#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub points: i32,
}

impl User {
    pub fn new(id: i32, first_name: &str, last_name: &str, email: &str, points: i32) -> User {
        User {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            points,
        }
    }
}
