#[derive(Debug, sqlx::FromRow)]
pub struct User{
    pub id : i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub address: String
}