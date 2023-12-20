use std::collections::HashMap;
use serenity::model::id::{GuildId, ChannelId, MessageId};
use crate::utils::db::DatabasePool;
pub struct SchedulerCache {
    // Maps guild_id to a tuple of (channel_id, react_message_id)
    pub cache: HashMap<GuildId, (ChannelId, MessageId)>,
}

impl SchedulerCache {
    pub async fn new(db_pool: &DatabasePool) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cache = HashMap::new();
        
        let conn = db_pool.0.get().await?;
        let rows = conn.query("SELECT guild_id, channel_id, react_message_id FROM botscheduler", &[]).await?;
        
        for row in rows {
            let guild_id: i64 = row.get(0);
            let channel_id: i64 = row.get(1);
            let react_message_id: i64 = row.get(2);
            cache.insert(
                GuildId(guild_id as u64),
                (ChannelId(channel_id as u64), MessageId(react_message_id as u64)),
            );
        }
        
        Ok(SchedulerCache { cache })
    }
    pub async fn update(&mut self) {

        //ToDo: Update the cache periodically or when changes are made
        // ...
    }

    pub fn get(&self, guild_id: GuildId) -> Option<&(ChannelId, MessageId)> {
        self.cache.get(&guild_id)
    }

    
}
