use chrono::DateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    comics::models::Comic,
    schema::{comic_genres, comic_genres_mapping},
};

#[derive(Queryable, Selectable, Identifiable, Debug, ToSchema, PartialEq)]
#[diesel(table_name = comic_genres)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Insertable, Queryable, Selectable, Identifiable, Associations, Debug, Clone)]
#[diesel(belongs_to(Comic))]
#[diesel(belongs_to(Genre))]
#[diesel(table_name = comic_genres_mapping)]
#[diesel(primary_key(comic_id, genre_id))]
pub struct GenreMapping {
    pub comic_id: Uuid,
    pub genre_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, TS, PartialEq)]
pub struct ComicGenre {
    pub id: i32,
    pub name: String,
}
