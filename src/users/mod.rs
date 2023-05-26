use axum::{http::StatusCode, response::IntoResponse, Json};

pub mod models;
pub mod routes;

#[derive(thiserror::Error, Debug)]
pub enum UsersError {
    #[error("internal server error")]
    InternalServerError,

    #[error("user not found")]
    UserNotFound,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("bad request")]
    BadRequest,

    #[error("user has no posts")]
    HasNoPosts,

    #[error("already logged in")]
    AlreadyLoggedIn,

    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    PoolError(#[from] diesel_async::pooled_connection::deadpool::PoolError),

    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),

    #[error("validation error: {0}")]
    Validator(#[from] garde::Errors),

    #[error("{0}")]
    Conflict(String),
}

impl IntoResponse for UsersError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:#?}", self);

        match self {
            UsersError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            UsersError::HasNoPosts => (StatusCode::NOT_FOUND, "user has no posts").into_response(),
            UsersError::BadRequest => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            UsersError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()).into_response(),
            UsersError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            UsersError::Diesel(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            UsersError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            UsersError::Argon2(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            UsersError::AlreadyLoggedIn => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            UsersError::Validator(errors) => {
                let errors = errors
                    .flatten()
                    .iter()
                    .map(|(path, error)| format!("{path}: {error}"))
                    .collect::<Vec<String>>();

                (StatusCode::BAD_REQUEST, Json(errors)).into_response()
            }
            UsersError::PoolError(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
