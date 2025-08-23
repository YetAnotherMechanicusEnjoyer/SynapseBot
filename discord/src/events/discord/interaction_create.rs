use poise::serenity_prelude::{
    self as serenity, ActionRowComponent, ButtonKind, CreateActionRow, CreateButton, CreateEmbed,
    CreateInteractionResponseMessage, Embed,
};

fn convert_embed_to_create_embed(embed: &Embed) -> CreateEmbed {
    let mut create_embed = serenity::CreateEmbed::new();

    if let Some(author) = &embed.author {
        let mut author_builder = serenity::CreateEmbedAuthor::new(&author.name);
        if let Some(icon_url) = &author.icon_url {
            author_builder = author_builder.icon_url(icon_url.clone());
        }
        if let Some(url) = &author.url {
            author_builder = author_builder.url(url.clone());
        }
        create_embed = create_embed.author(author_builder);
    }

    if let Some(color) = embed.colour {
        create_embed = create_embed.color(color);
    }

    if let Some(description) = &embed.description {
        create_embed = create_embed.description(description);
    }

    if let Some(footer) = &embed.footer {
        let mut footer_builder = serenity::CreateEmbedFooter::new(&footer.text);
        if let Some(icon_url) = &footer.icon_url {
            footer_builder = footer_builder.icon_url(icon_url.clone());
        }
        create_embed = create_embed.footer(footer_builder);
    }

    if let Some(image) = &embed.image {
        create_embed = create_embed.image(image.url.clone());
    }

    if let Some(timestamp) = &embed.timestamp {
        create_embed = create_embed.timestamp(timestamp);
    }

    if let Some(thumbnail) = &embed.thumbnail {
        create_embed = create_embed.thumbnail(thumbnail.url.clone());
    }

    if let Some(title) = &embed.title {
        create_embed = create_embed.title(title);
    }

    if let Some(url) = &embed.url {
        create_embed = create_embed.url(url);
    }

    let fields_vec: Vec<_> = embed
        .fields
        .iter()
        .map(|field| (field.name.clone(), field.value.clone(), field.inline))
        .collect();

    create_embed = create_embed.fields(fields_vec);

    create_embed
}

pub async fn interaction_create(ctx: serenity::Context, interaction: serenity::Interaction) {
    if let serenity::Interaction::Component(component) = interaction {
        let parts: Vec<&str> = component.data.custom_id.split('|').collect();
        if parts[0] == "Accept" || parts[0] == "Cancel" {
            println!("{}", parts[0]);
            let user_id = parts[1];
            let click_id = component.user.id.to_string();

            let embeds = component
                .message
                .embeds
                .iter()
                .map(convert_embed_to_create_embed)
                .collect();

            let new_components: Vec<CreateActionRow> = component
                .message
                .components
                .iter()
                .map(|action_row| {
                    let mut new_buttons = Vec::new();
                    for inner_component in &action_row.components {
                        if let ActionRowComponent::Button(button) = inner_component
                            && let ButtonKind::NonLink { custom_id, style } = &button.data
                        {
                            new_buttons.push(
                                CreateButton::new(custom_id.clone())
                                    .label(button.label.clone().unwrap())
                                    .style(style.to_owned())
                                    .emoji(button.emoji.clone().unwrap())
                                    .disabled(true),
                            );
                        }
                    }
                    CreateActionRow::Buttons(new_buttons)
                })
                .collect::<Vec<_>>();

            if user_id == click_id {
                component
                    .create_response(
                        &ctx,
                        serenity::CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new()
                                .embeds(embeds)
                                .components(new_components),
                        ),
                    )
                    .await
                    .expect("Failed to create response");
            }
        }
    }
}
