use std::time::SystemTime;

#[derive(Queryable)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub create_date: SystemTime,
}
