use common::*;

use crate::users::dtos::RegisterNewUserResponseDto;
use crate::AuthProvider;

use super::RegisterNewUserCommand;

#[async_trait::async_trait]
impl CommandHandler<RegisterNewUserCommand> for AuthProvider {
    type Output = RegisterNewUserResponseDto;
    async fn command(&self, cmd: RegisterNewUserCommand) -> AppResult<Self::Output> {
        register_new_user_command(self, cmd).await
    }
}

async fn register_new_user_command(provider: &AuthProvider, cmd: RegisterNewUserCommand) -> AppResult<RegisterNewUserResponseDto> {
    provider.validate(cmd).await?;
    todo!()

}
