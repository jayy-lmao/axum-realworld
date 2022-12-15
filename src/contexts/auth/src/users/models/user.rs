use common::*;

pub(in crate) struct User {
    username: SharedString,
    email: SharedString,
    password: SharedString,
}
