use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserAccount {
    pub uuid: Uuid,
    pub firstname: String,
    pub surname: String,
}