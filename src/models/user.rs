use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
    password: String,
    created_at: String,
    updated_at: String,
}

impl User{
    pub fn get_all_users() -> Vec<User> {
        vec![
            User {
                id: Uuid::new_v4().as_u128() as u64,
                name: String::from("André Borba"),
                email: String::from("test@test.com"),
                password: String::from("123456"),
                created_at: String::from("2023-01-01 00:00:00"),
                updated_at: String::from("2023-01-01 00:00:00"),
            },
            User {
                id: Uuid::new_v4().as_u128() as u64,
                name: String::from("Filipe Fonsêca"),
                email: String::from("test@testtest.com"),
                password: String::from("654321"),
                created_at: String::from("2023-01-01 00:00:00"),
                updated_at: String::from("2023-01-01 00:00:00"),
            },
        ]
    }
}
