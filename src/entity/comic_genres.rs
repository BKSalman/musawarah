//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "comic_genres")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub create_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::comics::Entity> for Entity {
    fn to() -> RelationDef {
        super::comics_genres_mapping::Relation::Comics.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::comics_genres_mapping::Relation::ComicGenres
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
