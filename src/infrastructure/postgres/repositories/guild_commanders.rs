use anyhow::Result;
use axum::async_trait;
use diesel::{insert_into, prelude::*};
use std::sync::Arc;

use crate::{
    domain::{
        entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity},
        repositories::guild_commanders::GuildCommandersRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::guild_commanders},
};

pub struct GuildCommanderPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl GuildCommanderPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl GuildCommandersRepository for GuildCommanderPostgres {
    async fn register(&self, guild_commander_entity: RegisterGuildCommanderEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(guild_commanders::table)
            .values(guild_commander_entity)
            .returning(guild_commanders::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = guild_commanders::table
            .filter(guild_commanders::username.eq(username))
            .select(GuildCommanderEntity::as_select())
            .first::<GuildCommanderEntity>(&mut conn)?;

        Ok(result)
    }
}
