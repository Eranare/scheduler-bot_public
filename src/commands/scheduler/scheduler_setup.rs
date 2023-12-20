use crate::{Context, Error};
use crate::DbPool;
use poise::serenity_prelude as serenity;
use serenity::{
    model::{
        channel::Message,
        channel::ReactionType,
    },
    utils::Colour,
};
//Scheduler_setup.
#[poise::command(slash_command)]
pub async fn scheduler_setup(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // Post the initial messages and get their IDs
    let react_message = post_message(ctx).await?;
    let timetable_message = post_timetable(ctx).await?;
    

    // Print out the IDs of the posted messages
    println!("Channel id: {}", ctx.channel_id());
    println!("Timetable message ID: {}", timetable_message.id);
    println!("Reaction message ID: {}", react_message.id);

        // Access the database pool from the context data
        let data = ctx.data();
        let db_pool = &ctx.data().db_pool;
        
        // Save the message IDs and relevant information to the database

        save_scheduler_info_to_database(
            &db_pool.0, 
            ctx.guild_id().unwrap().0, 
            ctx.channel_id().0, 
            timetable_message.id.0, 
            react_message.id.0
        ).await?;
    
    // For now, just confirm the action in the command response (or send a DM)
    if let Err(e) = ctx.say("Scheduler setup complete. Check the console for message IDs.").await {
        println!("Error sending confirmation: {}", e);
    }
    
    Ok(())
}


pub async fn post_timetable(
    ctx: Context<'_>,
) -> Result<Message, Error> {
    let channel_id = ctx.channel_id(); // Use the channel ID where the message is being posted

    let message = channel_id.send_message(&ctx.http(), |m| {
        m.embed(|e| {
            e.title("Timetable")
             .color(Colour::DARK_BLUE)
             .image("attachment://timetable.png") // Include the image in the embed
        })
        .add_file("/usr/local/bin/resources/images/timetable.png")
    }).await?;

    Ok(message)
}
    //.add_file("resources/images/timetable.png")

pub async fn post_message(
    ctx: Context<'_>,
) -> Result<Message, Error> {
    let channel_id = ctx.channel_id(); // Replace with your channel ID

    let message = channel_id.send_message(&ctx.http(), |m| {
        m.embed(|e| {
            e.title("Unavailability Schedule")
             .description("Please react to the emojis below corresponding to when you are NOT available:")
             .color(Colour::DARK_BLUE)
             .field("Monday", "🍕 2-5 PM\n🚀 7-10 PM", false)
             .field("Tuesday", "🎩 2-5 PM\n🐉 7-10 PM", false)
             .field("Wednesday", "🌮 2-5 PM\n🦄 7-10 PM", false)
             .field("Thursday", "🧀 2-5 PM\n🌛 7-10 PM", false)
             .field("Friday", "🎮 2-5 PM\n🍔 7-10 PM", false)
             .field("Saturday", "🍰 2-5 PM\n🐒 7-10 PM", false)
             .field("Sunday", "🍿 2-5 PM\n🍣 7-10 PM", false)
             // You can also add a thumbnail, image, footer, etc. if you want.
        })
    }).await?;

    // Emojis for timeslots
    let emojis = vec!["🍕", "🚀", "🎩", "🐉", "🌮", "🦄", "🧀", "🌛", "🎮", "🍔", "🍰", "🐒", "🍿", "🍣"];
    for emoji in emojis {
        message.react(&ctx.http(), ReactionType::Unicode(emoji.to_string())).await?;
    }

    Ok(message)
}


async fn save_scheduler_info_to_database(
    db_pool: &DbPool,
    guild_id: u64,
    channel_id: u64,
    timetable_message_id: u64,
    react_message_id: u64,
) -> Result<(), Error> {
    // Create an SQL query to insert or update the record in the botscheduler table
    let sql = "
        INSERT INTO botscheduler (guild_id, channel_id, timetable_message_id, react_message_id)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (guild_id) 
        DO UPDATE SET 
            channel_id = EXCLUDED.channel_id, 
            timetable_message_id = EXCLUDED.timetable_message_id,
            react_message_id = EXCLUDED.react_message_id;
    ";
    if guild_id == 0 || channel_id == 0 || timetable_message_id == 0 || react_message_id == 0 {
        return Err(Error::from("One or more IDs are zero."));
    }
    let conn = db_pool.get().await.map_err(|e| Box::new(e))?;
    conn.execute(sql, &[&(guild_id as i64), &(channel_id as i64), &(timetable_message_id as i64), &(react_message_id as i64)])
        .await
        .map_err(|e| Box::new(e))?;

    Ok(())
}
