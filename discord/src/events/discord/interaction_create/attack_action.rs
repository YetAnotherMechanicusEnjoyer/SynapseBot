use std::collections::HashMap;

use poise::serenity_prelude::{
    self as serenity, Colour, ComponentInteraction, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, MessageId,
};
use tokio::sync::MutexGuard;

use super::create_combat_ui;
use crate::{Error, GameState};

pub async fn attack_action(
    ctx: &serenity::Context,
    mut data: MutexGuard<'_, HashMap<MessageId, GameState>>,
    component: ComponentInteraction,
    mut game_state: GameState,
) -> Result<(), Error> {
    if component.user.id != game_state.turn {
        component
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("It's not your turn!")
                        .ephemeral(true),
                ),
            )
            .await
            .ok();
        return Ok(());
    }

    let (_attacker_id, defender_id) = if component.user.id == game_state.player1_id {
        (game_state.player1_id, game_state.player2_id)
    } else {
        (game_state.player2_id, game_state.player1_id)
    };

    let damage = 10;

    if defender_id == game_state.player1_id {
        game_state.player1_hp = game_state.player1_hp.saturating_sub(damage);
    } else {
        game_state.player2_hp = game_state.player2_hp.saturating_sub(damage);
    }

    let winner = if game_state.player1_hp == 0 {
        Some(game_state.player2_id)
    } else if game_state.player2_hp == 0 {
        Some(game_state.player1_id)
    } else {
        None
    };

    let next_turn = if game_state.turn == game_state.player1_id {
        game_state.player2_id
    } else {
        game_state.player1_id
    };
    game_state.turn = next_turn;

    let p1_user = game_state.player1_id.to_user(&ctx).await.unwrap();
    let p2_user = game_state.player2_id.to_user(&ctx).await.unwrap();

    let description = if let Some(winner_id) = winner {
        let winner_user = winner_id.to_user(&ctx).await.unwrap();
        format!(
            "{} defeated {}!",
            winner_user,
            if winner_id == p1_user.id {
                p2_user.clone()
            } else {
                p1_user.clone()
            }
        )
    } else {
        format!(
            "{} attacked {} for {} damage! It is now {}'s turn.",
            component.user,
            next_turn.to_user(&ctx).await.unwrap(),
            damage,
            next_turn.to_user(&ctx).await.unwrap()
        )
    };

    let new_embed = CreateEmbed::new()
        .title("Duel in Progress")
        .description(description)
        .field(
            format!("{}`s HP", p1_user.name),
            game_state.player1_hp.to_string(),
            true,
        )
        .field(
            format!("{}`s HP", p2_user.name),
            game_state.player2_hp.to_string(),
            true,
        )
        .color(Colour::from_rgb(0, 100, 255));

    let components = if winner.is_some() {
        data.remove(&component.message.id);
        vec![]
    } else {
        let next_user = next_turn.to_user(&ctx).await.unwrap();
        create_combat_ui(&next_user)
    };

    data.insert(component.message.id, game_state);

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
