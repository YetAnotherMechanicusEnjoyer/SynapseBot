use poise::serenity_prelude::{
    self as serenity, ButtonStyle, CreateActionRow, CreateButton, EmojiId, Interaction,
    ReactionType, User,
};

use crate::{Data, Error};

mod accept_duel;
mod attack_action;
mod cancel_duel;

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
        let data = framework.user_data().await.active_duels.lock().await;

        let game_state_option = data.get(&component.message.id).cloned();

        if let Some(game_state) = game_state_option {
            match component.data.custom_id.as_str() {
                "accept_duel" => accept_duel::accept_duel(ctx, component, game_state).await?,
                "cancel_duel" => cancel_duel::cancel_duel(ctx, data, component, game_state).await?,
                "attack_action" => {
                    attack_action::attack_action(ctx, data, component, game_state).await?
                }
                _ => {}
            }
        }
    }
    Ok(())
}
