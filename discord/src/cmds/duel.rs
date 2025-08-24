use crate::{Context, Error, GameState};
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
    let challenger_id = ctx.author().id;
    let opponent_id = user.id;

    if challenger_id == opponent_id || user.bot {
        return Err("You can't duel yourself or a bot!".into());
    }

    let initial_state = GameState {
        player1_id: challenger_id,
        player2_id: opponent_id,
        player1_hp: 100,
        player2_hp: 100,
        turn: challenger_id,
    };

    let thumbnail = user.avatar_url().unwrap_or("".into());

    let bot_img = ctx
        .framework()
        .bot_id
        .to_user(&ctx.http())
        .await?
        .avatar_url()
        .unwrap_or("".into());

    let author_img = ctx.author().avatar_url().unwrap_or("".into());

    let action_rows: Vec<CreateActionRow> = vec![CreateActionRow::Buttons(vec![
        CreateButton::new("accept_duel")
            .style(ButtonStyle::Success)
            .label("Accept")
            .emoji(ReactionType::Custom {
                animated: true,
                id: EmojiId::new(1127352603790032976),
                name: None,
            }),
        CreateButton::new("cancel_duel")
            .style(ButtonStyle::Danger)
            .label("Cancel")
            .emoji(ReactionType::Custom {
                animated: true,
                id: EmojiId::new(1127352919356882984),
                name: None,
            }),
    ])];

    let duel_message = ctx
        .send(CreateReply {
            embeds: vec![
                CreateEmbed::new()
                    .title("Duel Challenge")
                    .author(
                        CreateEmbedAuthor::new(ctx.author().display_name())
                            .icon_url(author_img.to_owned()),
                    )
                    .thumbnail(thumbnail.to_owned())
                    .description(format!(
                        "{} has challenged {} to a duel ! Press 'Accept' to take up arms, or 'Cancel' if your cowardice surpasses you.",
                        ctx.author(),
                        user
                    ))
                    .color(serenity::Colour::from_rgb(0, 100, 255))
                    .footer(
                        CreateEmbedFooter::new(ctx.invocation_string())
                            .icon_url(bot_img.to_owned()),
                    )
                    .timestamp(chrono::Utc::now()),
            ],
            components: Some(action_rows),
            ..Default::default()
        })
        .await?;

    ctx.data()
        .active_duels
        .lock()
        .await
        .insert(duel_message.message().await?.id, initial_state);

    Ok(())
}
