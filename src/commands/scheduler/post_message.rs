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


pub async fn post_message(
    ctx: Context<'_>,
) -> Result<Message, Error> {
    let channel_id = ChannelId(1175876059867131914); // Replace with your channel ID

    let message = channel_id.send_message(&ctx.discord().http, |m| {
        m.embed(|e| {
            e.title("Availability Schedule")
             .description("Please react to the emojis below corresponding to your availability:")
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
        message.react(&ctx.discord().http, ReactionType::Unicode(emoji.to_string())).await?;
    }

    Ok((message))
}