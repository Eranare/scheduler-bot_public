use poise::serenity_prelude as serenity;
use std::time::Duration;
use tokio::time::interval; // If you need interval for periodic tasks

use super::listeners;
use crate::{Data, Error, commands::get};


use crate::utils::scheduler::update::update_timetable_image_and_message;

//Listeners function 

async fn listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::ReactionAdd { add_reaction, .. } => {
            listeners::handle_reaction_add(ctx.clone(), add_reaction.clone(), data).await?;
        },
        poise::Event::ReactionRemove { removed_reaction, .. } => {
            listeners::handle_reaction_remove(ctx.clone(), removed_reaction.clone(), data).await?;
        },
        _ => {}
    }

    Ok(())
}

pub async fn run_framework(data: Data, token: &str) -> Result<(), Error> {
    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        commands: get(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(";".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(180))),
            ..Default::default()
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(listener(ctx, event, framework, data))
        },
        ..Default::default()
            
        })
        
        .intents(
            serenity::GatewayIntents::GUILD_VOICE_STATES
                | serenity::GatewayIntents::privileged()
                | serenity::GatewayIntents::non_privileged()
                | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .token(token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                //Setup
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let ctx_clone = ctx.clone();
                let data_clone = data.clone(); 
                
                tokio::spawn(async move {
                    let mut interval = interval(Duration::from_secs(300)); // redraw image if needed every 5 minutes
                    loop {
                        interval.tick().await;
                        let mut counter = data_clone.update_counter.lock().await;
                        if *counter > 0 {
                            if let Err(e) = update_timetable_image_and_message(&ctx_clone, &data_clone).await {
                                println!("Error updating timetable: {}", e);
                            }
                            *counter = 0; // Reset the counter
                        }
                    }
                });

                Ok(data)
            })
        })
        .run()
        .await?;

    Ok(())
}