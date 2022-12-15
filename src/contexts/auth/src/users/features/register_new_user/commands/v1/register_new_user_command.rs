use common::*;
pub struct RegisterNewUserCommand {
    pub username: SharedString,
    pub email: SharedString,
    pub password: SharedString,
}
