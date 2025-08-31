use poise::serenity_prelude::{
    self as serenity, ComponentInteraction, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use super::create_combat_ui;
use crate::{Error, GameState};

pub async fn accept_duel(
    ctx: &serenity::Context,
    component: ComponentInteraction,
    game_state: GameState,
) -> Result<(), Error> {
    if component.user.id != game_state.player2_id {
        component
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("This duel challenge is not for you!")
                        .ephemeral(true),
                ),
            )
            .await
            .ok();
        return Ok(());
    }

    let p1_user = game_state.player1_id.to_user(&ctx).await.unwrap();
    let p2_user = game_state.player2_id.to_user(&ctx).await.unwrap();
    let new_embed = CreateEmbed::new()
        .title("Duel Started!")
        .description(format!(
            "{p1_user} vs {p2_user}\n\nIt is {p1_user}'s turn to attack!",
        ))
        .field(
            format!("{}`s HP", p1_user.name),
            game_state.player1_hp.to_string(),
            true,
        )
        .field(
            format!("{}`s HP", p2_user.name),
            game_state.player2_hp.to_string(),
            true,
        );

    let components = create_combat_ui(&p1_user);

    component
        .create_response(
            &ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embeds(vec![new_embed])
                    .components(components),
            ),
        )
        .await
        .ok();
    Ok(())
}
