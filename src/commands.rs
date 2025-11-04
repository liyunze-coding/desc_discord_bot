use poise::CreateReply;
use serenity::all::CreateEmbed;

use crate::{Context, Error};
use std::time::Instant;

/// Show this help menu
#[poise::command(track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Ping the bot
// #[poise::command(slash_command)]
// pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
//     let start = Instant::now();

//     let msg = ctx.say("Pinging... ").await?;

//     let elapsed_ms = start.elapsed().as_millis();

//     msg.edit(
//         ctx,
//         CreateReply::default().content(format!("Pong!\nTime elapsed: {elapsed_ms} ms")),
//     )
//     .await?;

//     Ok(())
// }

/// Ping the bot
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    let start_embed_msg = CreateEmbed::new().title("Pinging...").color();
}
