use common::*;

#[derive(Serialize, Deserialize)]
pub struct RegisterNewUserResponseDto {
    pub id: i32,
    pub email: String,
}
