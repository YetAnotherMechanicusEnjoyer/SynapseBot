use crate::{Context, Error};
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
};

/// Displays available commands
#[poise::command(slash_command, prefix_command, category = "Help")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "More information about a specific command"] cmd: Option<String>,
) -> Result<(), Error> {
    let cmds = super::get_all_commands();
    let mut fields;
    let title;
    let description;
    let mut categories = vec![];

    if let Some(cmd) = cmd {
        title = format!("`{cmd}` command");
        description = None;

        let c = match cmds.iter().find(|c| c.name == cmd) {
            Some(c) => c,
            None => {
                ctx.say(format!("Error: Command `{cmd}` no found.")).await?;
                return Ok(());
            }
        };
        let name = c.name.to_owned();
        let desc = match c.description.to_owned() {
            Some(d) => d,
            None => "None".into(),
        };
        let guild = c.guild_only.to_owned();
        let dm = c.dm_only.to_owned();

        fields = vec![
            ("Name", format!("`{name}`"), false),
            ("Description", format!("`{desc}`"), false),
            ("Guild Only", format!("`{guild}`"), true),
            ("DM Only", format!("`{dm}`"), true),
        ];
    } else {
        let prefix = match ctx.framework().options().prefix_options.prefix.to_owned() {
            Some(p) => p,
            None => "None".into(),
        };

        cmds.iter().for_each(|c| {
            if let Some(category) = c.category.to_owned()
                && !categories.contains(&category)
            {
                categories.push(category)
            }
        });

        title = "Available commands".to_string();
        description = Some(format!(
            "Prefix: `{}`\nAvailable commands: `{}`\nAvailable categories: `{}`",
            prefix,
            cmds.iter().len(),
            categories.iter().len()
        ));

        fields = vec![];

        categories.sort();
        categories.iter().for_each(|cat| {
            let commands = cmds.iter().filter(|c| {
                if let Some(category) = c.category.to_owned() {
                    &category == cat
                } else {
                    false
                }
            });
            let mut value = String::new();
            for c in commands {
                let name = c.name.to_owned();
                let desc = match c.description.to_owned() {
                    Some(d) => d,
                    None => "None".into(),
                };
                value.push_str(&format!("`{name}`: **{desc}**\n"));
            }

            fields.push((cat, value, false));
        });
    }

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

    let mut embed = CreateEmbed::new()
        .title(title)
        .author(CreateEmbedAuthor::new(author).icon_url(author_img.to_owned()))
        .thumbnail(bot_img.to_owned())
        .fields(fields)
        .color(serenity::Colour::from_rgb(color.0, color.1, color.2))
        .footer(CreateEmbedFooter::new(footer).icon_url(bot_img.to_owned()))
        .timestamp(chrono::Utc::now());

    if let Some(description) = description {
        embed = embed.to_owned().description(description);
    }

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;
    Ok(())
}
