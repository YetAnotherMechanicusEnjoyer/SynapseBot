use poise::serenity_prelude::{
    self as serenity, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, EmojiId, Interaction,
    ReactionType, User,
};

use crate::{Data, Error};

fn create_combat_ui(turn_user: &User) -> Vec<CreateActionRow> {
    let attack_button = CreateButton::new("attack_action")
        .style(ButtonStyle::Primary)
        .label("Attack!")
        .emoji(ReactionType::Unicode("⚔️".to_string()))
        .disabled(false);

    let disabled_button = CreateButton::new("wait_turn")
        .style(ButtonStyle::Secondary)
        .label(format!("Waiting for {}...", turn_user.name))
        .emoji(ReactionType::Custom {
            animated: true,
            id: EmojiId::new(983173429224157254),
            name: None,
        })
        .disabled(true);

    let buttons = vec![attack_button, disabled_button];

    vec![CreateActionRow::Buttons(buttons)]
}

pub async fn interaction_create(
    ctx: &serenity::Context,
    interaction: Interaction,
    framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let serenity::Interaction::Component(component) = interaction {
        let mut data = framework.user_data().await.active_duels.lock().await;

        let game_state_option = data.get(&component.message.id).cloned();

        if let Some(mut game_state) = game_state_option {
            match component.data.custom_id.as_str() {
                "accept_duel" => {
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
                }
                "cancel_duel" => {
                    if component.user.id != game_state.player1_id
                        && component.user.id != game_state.player2_id
                    {
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
                }
                "attack_action" => {
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

                    let (_attacker_id, defender_id) = if component.user.id == game_state.player1_id
                    {
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
                        );

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
                }
                _ => {}
            }
        }
    }
    Ok(())
}
