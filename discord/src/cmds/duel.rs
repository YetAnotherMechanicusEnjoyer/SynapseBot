use crate::{Context, Error};
use poise::{
    CreateReply,
    serenity_prelude::{
        self as serenity, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed,
        CreateEmbedAuthor, CreateEmbedFooter, EmojiId, ReactionType,
    },
};

/// Duel another user
#[poise::command(
    slash_command,
    prefix_command,
    category = "RPG",
    guild_only,
    broadcast_typing
)]
pub async fn duel(
    ctx: Context<'_>,
    #[description = "The user to duel against"] user: serenity::User,
) -> Result<(), Error> {
    let thumbnail = user.avatar_url().unwrap_or("".into());

    let bot_img = ctx
        .framework()
        .bot_id
        .to_user(&ctx.http())
        .await?
        .avatar_url()
        .unwrap_or("".into());

    let author_img = ctx.author().avatar_url().unwrap_or("".into());

    let color = ctx.data().color;

    let action_rows: Vec<CreateActionRow> = vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("Accept|{}", user.id))
            .style(ButtonStyle::Success)
            .label("Accept")
            .emoji(ReactionType::Custom {
                animated: true,
                id: EmojiId::new(1127352603790032976),
                name: None,
            }),
        CreateButton::new(format!("Cancel|{}|{}", user.id, ctx.author().id))
            .style(ButtonStyle::Danger)
            .label("Cancel")
            .emoji(ReactionType::Custom {
                animated: true,
                id: EmojiId::new(1127352919356882984),
                name: None,
            })
            .disabled(false),
    ])];

    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::new()
                .title("Duel")
                .author(
                    CreateEmbedAuthor::new(ctx.author().display_name())
                        .icon_url(author_img.to_owned()),
                )
                .description(format!("Mentionned: {user}"))
                .thumbnail(thumbnail.to_owned())
                .color(serenity::Colour::from_rgb(color.0, color.1, color.2))
                .footer(
                    CreateEmbedFooter::new(ctx.invocation_string()).icon_url(bot_img.to_owned()),
                )
                .timestamp(chrono::Utc::now()),
        ],
        components: Some(action_rows),
        ..Default::default()
    })
    .await?;
    Ok(())
}
