//Listeners/mod.rs
use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use lazy_static::lazy_static;
use percent_encoding::percent_decode_str;

const TIMESLOTS: [&str; 14] = [
    "Monday 2-5 PM", "Monday 7-10 PM", "Tuesday 2-5 PM", "Tuesday 7-10 PM",
    "Wednesday 2-5 PM", "Wednesday 7-10 PM", "Thursday 2-5 PM", "Thursday 7-10 PM",
    "Friday 2-5 PM", "Friday 7-10 PM", "Saturday 2-5 PM", "Saturday 7-10 PM",
    "Sunday 2-5 PM", "Sunday 7-10 PM",
];
pub async fn handle_reaction_add(
    ctx: serenity::Context, // Use owned serenity::Context
    add_reaction: serenity::model::channel::Reaction, // Use owned Reaction
    data: &Data // Pass data as a reference
) -> Result<(), Error> {
    println!("Reaction added to message ID: {}", add_reaction.message_id);

    // Check if the reaction is to the specific message
    if let Some(guild_id) = add_reaction.guild_id {
        if let Some((_, target_message_id)) = data.scheduler_cache.get(guild_id) {
            if add_reaction.message_id == *target_message_id {
                let conn = data.db_pool.0.get().await?;

                    // Retrieve user ID and guild ID from the reaction
                let user_id = add_reaction.user_id.unwrap().0 as i64; // Convert to i64 or your preferred type
                let guild_id = add_reaction.guild_id.unwrap().0 as i64;
                
                    // Check if user exists in 'users' table, if not create new entry
                    let user_exists: bool = conn
                    .query_one(
                        "SELECT EXISTS(SELECT 1 FROM users WHERE guild_id = $1 AND user_id = $2)",
                        &[&guild_id, &user_id],
                    )
                    .await?
                    .get(0);

                    if !user_exists {
                        conn.execute(
                            "INSERT INTO users (guild_id, user_id) VALUES ($1, $2)",
                            &[&guild_id, &user_id],
                        ).await?;
                    
                        // Create 14 entries in user_time_slots for the new user
                        let time_slots = vec![
                            "Monday 2-5 PM", "Monday 7-10 PM", "Tuesday 2-5 PM", "Tuesday 7-10 PM", 
                            "Wednesday 2-5 PM", "Wednesday 7-10 PM", "Thursday 2-5 PM", "Thursday 7-10 PM", 
                            "Friday 2-5 PM", "Friday 7-10 PM", "Saturday 2-5 PM", "Saturday 7-10 PM", 
                            "Sunday 2-5 PM", "Sunday 7-10 PM"
                        ];
                    
                        // Retrieve the user_record_id (id from users table) for the inserted user
                        let user_record_id: i64 = conn.query_one(
                            "SELECT id FROM users WHERE guild_id = $1 AND user_id = $2",
                            &[&guild_id, &user_id],
                        ).await?.get(0);
                    
                        // Prepare the SQL statement for bulk insert
                        let mut sql = "INSERT INTO user_time_slots (user_record_id, day, available) VALUES ".to_string();
                        for (index, slot) in time_slots.iter().enumerate() {
                            sql.push_str(&format!("({}, '{}', true)", user_record_id, slot));
                            if index < time_slots.len() - 1 {
                                sql.push_str(",");
                            }
                        }
                    
                        // Execute the SQL statement
                        conn.execute(&sql, &[]).await?;
                    }
                    // ...
                    if let Some(reaction_index) = find_reaction_index(&ctx, &add_reaction).await? {
                        let time_slot = TIMESLOTS[reaction_index];
                        // Update the specific time slot for the user
                        conn.execute(
                            "UPDATE user_time_slots 
                            SET available = false 
                            WHERE user_record_id = (SELECT id FROM users WHERE guild_id = $1 AND user_id = $2)
                            AND day = $3",
                            &[&guild_id, &user_id, &time_slot],
                        ).await?;
                        /*
                        if let Err(e) = crate::utils::scheduler::generate_timetable::generate_timetable_image(&data.db_pool.0, guild_id).await {
                            println!("Error generating timetable image: {}", e);
                        } */
                println!("Reaction index: {}, Updated availability for user ID: {}", reaction_index, user_id);
                    }
            
            }
        }
    }
    let mut counter = data.update_counter.lock().await;
    *counter = (*counter + 1).min(2); // Ensure the counter doesn't exceed 2

    Ok(())
}

pub async fn handle_reaction_remove(
    ctx: serenity::Context, // Use owned serenity::Context
    removed_reaction: serenity::model::channel::Reaction, // Use owned Reaction
    data: &Data // Pass data as a reference
) -> Result<(), Error> {
    println!("Reaction removed from message ID: {}", removed_reaction.message_id);
    // Retrieve the target message ID from the cache
    if let Some(guild_id) = removed_reaction.guild_id {
        if let Some((_, target_message_id)) = data.scheduler_cache.get(guild_id) {
            if removed_reaction.message_id == *target_message_id {
                // Assuming emoji_map and db_pool are available here
                let user_id = removed_reaction.user_id.unwrap().0 as i64;
                let guild_id = removed_reaction.guild_id.unwrap().0 as i64;

                let conn = data.db_pool.0.get().await?;

                if let Some(reaction_index) = find_reaction_index(&ctx, &removed_reaction).await? {
                    let time_slot = TIMESLOTS[reaction_index];
                    // Update the specific time slot for the user
                    conn.execute(
                        "UPDATE user_time_slots 
                        SET available = true 
                        WHERE user_record_id = (SELECT id FROM users WHERE guild_id = $1 AND user_id = $2)
                        AND day = $3",
                        &[&guild_id, &user_id, &time_slot],
                    ).await?;
                    /*
                    if let Err(e) = crate::utils::scheduler::generate_timetable::generate_timetable_image(&data.db_pool.0, guild_id).await {
                        println!("Error generating timetable image: {}", e);
                    }*/
                }

                println!("Updated availability for user ID: {}", user_id);

               
            }
        }
    }
    let mut counter = data.update_counter.lock().await;
    *counter = (*counter + 1).min(2); // Ensure the counter doesn't exceed 2

    Ok(())
}
// Define other helper functions like map_emoji_to_timeslot, check_and_insert_user, update_user_time_slots.
async fn find_reaction_index(
    ctx: &serenity::Context, 
    reaction: &serenity::model::channel::Reaction
) -> Result<Option<usize>, Error> {
    let message = reaction.message(&ctx.http).await.map_err(|e| Box::new(e))?;

    for (index, reaction_type) in message.reactions.iter().enumerate() {
        if let serenity::model::channel::ReactionType::Unicode(emoji) = &reaction_type.reaction_type {
            let decoded_emoji = percent_decode_str(&reaction.emoji.as_data()).decode_utf8_lossy().to_string();
    
            println!("Comparing: emoji in message: {}, emoji in reaction: {}", emoji, decoded_emoji);
    
            if emoji == &decoded_emoji {
                println!("Matching index found: {}", index);
                return Ok(Some(index));
            }
        }
    }
    
    println!("No reaction index found for reaction: {:?}", reaction.emoji);
    Ok(None)
}

lazy_static! {
    static ref EMOJI_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("🍕", "Monday 2-5 PM");
        m.insert("🚀", "Monday 7-10 PM");
        m.insert("🎩", "Tuesday 2-5 PM");
        m.insert("🐉", "Tuesday 7-10 PM");
        m.insert("🌮", "Wednesday 2-5 PM");
        m.insert("🦄", "Wednesday 7-10 PM");
        m.insert("🧀", "Thursday 2-5 PM");
        m.insert("🌛", "Thursday 7-10 PM");
        m.insert("🎮", "Friday 2-5 PM");
        m.insert("🍔", "Friday 7-10 PM");
        m.insert("🍰", "Saturday 2-5 PM");
        m.insert("🐒", "Saturday 7-10 PM");
        m.insert("🍿", "Sunday 2-5 PM");
        m.insert("🍣", "Sunday 7-10 PM");
        m
    };
}

