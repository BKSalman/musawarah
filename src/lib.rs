use std::{fmt::Display, fs};

use axum::{extract::FromRef, response::IntoResponse};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use once_cell::sync::OnceCell;
use s3::interface::Storage;
use serde::{Deserialize, Serialize};
use tower_cookies::cookie::Key;
use ts_rs::TS;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    IntoParams, Modify, OpenApi, ToSchema,
};
use uuid::Uuid;

pub mod auth;
pub mod chapters;
pub mod comic_genres;
pub mod comics;
pub mod common;
pub mod migrations;
pub mod s3;
pub mod schema;
pub mod sessions;
pub mod users;
pub mod utils;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub cookie_secret: String,
}

impl Config {
    pub fn load_config() -> Result<Self, ConfigError> {
        let config_file = fs::read_to_string("config.toml")?;
        toml::from_str::<Config>(&config_file).map_err(Into::into)
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: Pool<AsyncPgConnection>,
    pub storage: Storage,
}

pub static COOKIES_SECRET: OnceCell<Key> = OnceCell::new();

#[derive(OpenApi)]
#[openapi(
    paths(
        users::routes::create_user,
        users::routes::login,
        users::routes::get_user_comics,
        users::routes::get_user,
        comics::routes::create_comic,
        comics::routes::update_comic,
        comics::routes::delete_comic,
        comics::routes::get_comic,
        comics::routes::get_comics,
        chapters::routes::create_chapter,
        chapters::routes::create_chapter_page,
        chapters::routes::get_chapters,
        chapters::routes::get_chapter,
        chapters::routes::delete_chapter,
        chapters::routes::delete_chapter_page,
        chapters::routes::update_chapter,
        // chapters::routes::update_chapter_page,
        comic_genres::routes::get_genres,
        comic_genres::routes::create_genre,
        comic_genres::routes::update_genre,
        comic_genres::routes::delete_genre,
    ),
    components(
        schemas(common::models::ImageResponse),
        schemas(comics::models::CreateComic),
        schemas(comics::models::UpdateComic),
        schemas(comics::models::ComicResponse),
        schemas(comic_genres::models::ComicGenre),
        schemas(chapters::models::CreateChapter),
        schemas(chapters::models::UpdateChapter),
        schemas(chapters::models::CreateChapterPage),
        schemas(chapters::models::ChapterResponse),
        schemas(chapters::models::ChapterResponseBrief),
        schemas(chapters::models::ChapterPageResponse),
        schemas(users::models::UserResponse),
        schemas(users::models::UserClaims),
        schemas(users::models::CreateUser),
        schemas(users::models::UserLogin),
        schemas(users::models::UserToken),
        schemas(ErrorResponse),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Users API"),
        (name = "Comics API"),
        (name = "Chapters API"),
        (name = "Comic Genres API"),
    )
)]
pub struct ApiDoc;

#[derive(Debug, Deserialize, IntoParams)]
pub struct PaginationParams {
    #[serde(default = "Uuid::nil")]
    min_id: Uuid,
    #[serde(default = "Uuid::max")]
    max_id: Uuid,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Default, TS)]
#[ts(export)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<Vec<String>>,
}

impl ErrorResponse {
    pub fn new(error_message: impl Display) -> Self {
        Self {
            error: error_message.to_string(),
            ..Default::default()
        }
    }

    pub fn with_details(error_message: impl Display, details: Vec<impl Display>) -> Self {
        Self {
            error: error_message.to_string(),
            details: Some(details.iter().map(|s| s.to_string()).collect()),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        serde_json::to_string(&self)
            .expect("ErrorResponse as json")
            .into_response()
    }
}

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            let http_auth_scheme = HttpBuilder::new().scheme(HttpAuthScheme::Basic).build();
            components.add_security_scheme("auth", SecurityScheme::Http(http_auth_scheme))
        }
    }
}

pub trait Rating {
    fn rating(&self) -> f64;
}
