use std::collections::HashMap;

use poise::serenity_prelude::{
    self as serenity, ComponentInteraction, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, MessageId,
};
use tokio::sync::MutexGuard;

use crate::{Error, GameState};

pub async fn cancel_duel(
    ctx: &serenity::Context,
    mut data: MutexGuard<'_, HashMap<MessageId, GameState>>,
    component: ComponentInteraction,
    game_state: GameState,
) -> Result<(), Error> {
    if component.user.id != game_state.player1_id && component.user.id != game_state.player2_id {
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

    data.remove(&component.message.id);
    let new_embed = CreateEmbed::new()
        .title("Duel Cancelled")
        .description("The duel has been cancelled.");

    component
        .create_response(
            &ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embeds(vec![new_embed])
                    .components(vec![]),
            ),
        )
        .await
        .ok();
    Ok(())
}
