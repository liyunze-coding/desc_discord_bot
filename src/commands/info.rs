use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, GuildId, User};

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
            extra_text_at_bottom: "made using Serenity Poise framework in Rust and with ❤️",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Ping the bot
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    let embed_color = Colour::DARK_GREY;

    let start_embed_msg = CreateEmbed::new().title("Pinging...").color(embed_color);

    let first_reply: CreateReply = CreateReply::default().embed(start_embed_msg);

    let msg = ctx.send(first_reply).await?;

    // record ping
    let elapsed_ms = start.elapsed().as_millis();

    // create the 2nd embed
    let result_embed_msg = CreateEmbed::new()
        .title("Pong!")
        .description(format!("{} ms", elapsed_ms))
        .color(embed_color);

    msg.edit(ctx, CreateReply::default().embed(result_embed_msg))
        .await?;

    Ok(())
}

/// Display user's information
#[poise::command(slash_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "Specific user to show information about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: Option<User>,
) -> Result<(), Error> {
    let discord_user: User = user.unwrap_or(ctx.author().to_owned());
    let discord_member = GuildId::member(ctx.guild_id().unwrap(), ctx, discord_user.id).await?;

    let embed_color = discord_user.accent_colour.unwrap_or(Colour::DARK_GREY);

    let member_created_at = discord_user.created_at().format("%d/%m/%Y %I:%M %p");
    let member_joined_at = discord_member
        .joined_at
        .unwrap()
        .format("%d/%m/%Y %I:%M %p");

    let user_id = &discord_user.id;
    let username = &discord_user.name;

    let nickname = &discord_member.nick.unwrap_or(username.to_owned());

    let role_ids = discord_member.roles;
    let role_mentions: Vec<String> = role_ids
        .into_iter()
        .map(|role_id| format!("<@&{}>", role_id))
        .collect();

    let user_avatar_url = discord_user
        .avatar_url()
        .unwrap_or(discord_user.default_avatar_url());

    let result_embed_msg = CreateEmbed::new()
        .thumbnail(user_avatar_url)
        .color(embed_color)
        .title("User Info")
        .description(format!(
            "**ID**: {} \n\
            **Display Name**: {}\n\
            **Username**: {}\n\
            **Created At**: {}\n\
            **Joined At**: {}\n\
            **Roles**: {}
        ",
            user_id,
            nickname,
            username,
            member_created_at,
            member_joined_at,
            role_mentions.join(" ")
        ));

    ctx.send(CreateReply::default().embed(result_embed_msg))
        .await?;

    Ok(())
}

/// Display server's information
#[poise::command(slash_command)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    let embed_color = Colour::DARK_GREY;

    let partial_guild = ctx.partial_guild().await.unwrap();
    let member_count = {
        let guild = ctx.guild();
        guild.as_deref().unwrap().member_count
    };

    let server_description_option = &partial_guild.description;

    let server_id = &partial_guild.id;
    let server_name = &partial_guild.name;
    // let no_of_members = &discord_server.approximate_member_count.unwrap();
    let owner_id = &partial_guild.owner_id;

    let server_description = server_description_option.as_deref().unwrap_or("N/A");

    let server_icon = &partial_guild.icon_url().unwrap_or_default();

    let roles: &Vec<String> = &partial_guild
        .roles
        .into_keys()
        .filter(|role_id| format!("{}", role_id) != format!("{}", server_id))
        .map(|role_id| format!("<@&{}>", role_id))
        .collect();

    let rules_channel_id = &partial_guild.rules_channel_id;

    let rules_channel = if rules_channel_id.is_none() {
        "N/A"
    } else {
        &format!("<#{}>", rules_channel_id.unwrap())
    };

    let result_embed_msg = CreateEmbed::new()
        .thumbnail(server_icon)
        .title("Server Info")
        .description(format!(
            "
            **Name**: {}\n\
            **ID**: {}\n\
            **Owner**: <@{}>\n\
            **Description**: {}\n\
            **Rules Channel**: {}\n\
            **No. of Members**: {}\n\
            **Roles**: {}
            ",
            server_id,
            server_name,
            owner_id,
            server_description,
            rules_channel,
            member_count,
            roles.join(" ")
        ))
        .color(embed_color);

    ctx.send(CreateReply::default().embed(result_embed_msg))
        .await?;

    Ok(())
}
