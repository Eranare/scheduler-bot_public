use poise::serenity_prelude as serenity;
use serenity::{
    model::{
        channel::Message,
        id::ChannelId,
        channel::ReactionType,
    },
    utils::Colour,
};
use crate::{Context, Error};


pub async fn post_timetable(
    ctx: Context<'_>,
) -> Result<Message, Error> {
    let channel_id = ChannelId(1175876059867131914); // Replace with your channel ID

    let message = channel_id.send_message(&ctx.discord().http, |m| {
        m.embed(|e| {
            e.title("Timetable")
             .description("will be updated with the schedule")
             .color(Colour::DARK_BLUE)
             .field("image here", false)

             // You can also add a thumbnail, image, footer, etc. if you want.
        })
    }).await?;
    Ok((message))
}