use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct PooledConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PooledConn {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        match request.guard::<&State<Pool>>().await {
            request::Outcome::Success(state) => match state.get() {
                Ok(conn) => Outcome::Success(PooledConn(conn)),
                Err(_) => Outcome::Error((Status::ServiceUnavailable, ())),
            },
            _ => Outcome::Error((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for PooledConn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
