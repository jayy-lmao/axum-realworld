use common::*;

use crate::users::features::register_new_user::commands::v1::RegisterNewUserCommand;
use crate::users::features::register_new_user::dtos::v1::RegisterNewUserRequestDto;

impl From<RegisterNewUserRequestDto> for RegisterNewUserCommand {
    fn from(dto: RegisterNewUserRequestDto) -> Self {
        Self {
            username: SharedString::from(dto.username),
            email: SharedString::from(dto.email),
            password: SharedString::from(dto.password),
        }
    }
}
