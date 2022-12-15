use common::axum::*;
use common::axum::routing::post;
use common::axum::http::StatusCode;
use common::axum::response::IntoResponse;

use crate::users::dtos::RegisterNewUserResponseDto;
use crate::users::features::RegisterNewUserRequestDto;


pub fn routes(base_route: &str) -> Router {
    Router::new().route(
     format!("{}{}", base_route, "/users").as_str(), post(register_user))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#registration
async fn register_user(
    Json(req): Json<RegisterNewUserRequestDto>,
    ) -> impl IntoResponse  {
        let user = RegisterNewUserResponseDto { id: todo!(), email: todo!() };
        (StatusCode::CREATED, Json(user))

}
