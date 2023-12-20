use crate::utils::scheduler::generate_timetable::generate_timetable_image;
use crate::{Data, Error};
use serenity::prelude::Context;
use serenity::model::id::{GuildId, ChannelId, MessageId};
use chrono::Utc;

pub async fn update_timetable_image_and_message(ctx: &Context, data: &Data) -> Result<(), Error> {
    let conn = data.db_pool.0.get().await.map_err(|e| Box::new(e))?;

    // Retrieve all guilds, their channel IDs, and current timetable message IDs
    let guilds_info = conn.query(
        "SELECT guild_id, channel_id, timetable_message_id FROM botscheduler", 
        &[]
    ).await.map_err(|e| Box::new(e))?;

    for row in guilds_info {
        let guild_id: i64 = row.get(0);
        let channel_id: i64 = row.get(1);
        let current_timetable_message_id: i64 = row.get(2);

        // Generate the new timetable image for the guild
        let image_path = generate_timetable_image(&data.db_pool.0, guild_id).await?;

        // Delete the old message
        if let Ok(channel) = ctx.http.get_channel(channel_id as u64).await {
            let _ = channel.id().delete_message(&ctx.http, current_timetable_message_id as u64).await;
        }

        

        // Get the current UTC timestamp
        let timestamp = Utc::now().timestamp();
        let title = format!("Timetable Updated: <t:{}>", timestamp);

        // Post the new timetable message
        let new_message = ChannelId(channel_id as u64).send_message(&ctx.http, |m| {
            m.add_file(&*image_path) // Use a reference to the String
             .embed(|e| e.title(&title).image(format!("attachment://{}", image_path)))
        }).await?;

        // Update the timetable message ID in the database
        conn.execute(
            "UPDATE botscheduler SET timetable_message_id = $1 WHERE guild_id = $2",
            &[&(new_message.id.0 as i64), &guild_id]
        ).await.map_err(|e| Box::new(e))?;
    }

    Ok(())
}

