use common::*;

#[derive(Deserialize)]
pub struct RegisterNewUserRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}
