use common::*;

use crate::users::dtos::RegisterNewUserResponseDto;
use crate::users::exceptions::AuthContextError;
use crate::{users::features::RegisterNewUserRequestDto, AuthProvider};

use super::RegisterNewUserCommand;

#[async_trait::async_trait]
impl CommandHandler<RegisterNewUserCommand> for AuthProvider {
    type Output = RegisterNewUserResponseDto;
    async fn command(&self, cmd: RegisterNewUserCommand) -> AppResult<Self::Output> {
        register_new_user_command(self, cmd).await
    }
}

async fn register_new_user_command(provider: &AuthProvider, cmd: RegisterNewUserCommand) -> AppResult<RegisterNewUserResponseDto> {
    todo!()

}
