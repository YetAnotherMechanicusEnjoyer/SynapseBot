use crate::{Context, Error};
use chrono::Utc;
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
};

/// Displays latency
#[poise::command(slash_command, prefix_command, category = "Misc")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let gateway_latency = ctx.ping().await.as_millis();

    let interaction_timestamp = ctx.created_at();
    let latency_ms = Utc::now()
        .signed_duration_since(*interaction_timestamp)
        .num_milliseconds();

    let footer = ctx.invocation_string();

    let bot = ctx.framework().bot_id.to_user(&ctx.http()).await?;
    let bot_img = match bot.avatar_url() {
        Some(str) => str,
        None => "".into(),
    };

    let author = ctx.author().display_name();
    let author_img = match ctx.author().avatar_url() {
        Some(str) => str,
        None => "".into(),
    };

    let color = ctx.data().color;

    let fields = vec![
        (
            "Gateway latency",
            format!("**`{gateway_latency}`ms**"),
            true,
        ),
        ("User Latency", format!("**`{latency_ms}`ms**"), true),
        (
            "Disclaimer",
            "-# Negative values are due to Discord's shitty API, not the bot.".into(),
            false,
        ),
    ];
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::new()
                .title("Pong !")
                .author(CreateEmbedAuthor::new(author).icon_url(author_img.to_owned()))
                .thumbnail(bot_img.to_owned())
                .fields(fields)
                .color(serenity::Colour::from_rgb(color.0, color.1, color.2))
                .footer(CreateEmbedFooter::new(footer).icon_url(bot_img.to_owned()))
                .timestamp(chrono::Utc::now()),
        ],
        ..Default::default()
    })
    .await?;
    Ok(())
}
