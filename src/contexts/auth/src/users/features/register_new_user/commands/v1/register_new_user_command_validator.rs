
use common::*;

use crate::users::dtos::RegisterNewUserResponseDto;
use crate::AuthProvider;

use super::RegisterNewUserCommand;

#[async_trait::async_trait]
impl Validate<RegisterNewUserCommand> for AuthProvider {
    async fn validate(&self, cmd: RegisterNewUserCommand) -> AppResult<()> {
        validate_register_new_user_command(self, cmd).await
    }
}

async fn validate_register_new_user_command(provider: &AuthProvider, cmd: RegisterNewUserCommand) -> AppResult<()> {
    todo!()

}
