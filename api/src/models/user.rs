#[derive(Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub firstname: String,
    pub lastname: String,
}