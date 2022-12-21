use common::*;

#[derive(Clone)]
pub struct AuthProvider {
    db: Pool<Postgres>,
}
