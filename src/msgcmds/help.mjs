import Discord from "discord.js"

export const command = {
  name: "help",
  description: "Displays available commands.",
  permission: "None",
  context: "None",
  category: "Help",
  options: [
    {
      type: "string",
      name: "cmd",
      description: "More information about a specific command",
      autocomplete: true,
      required: false
    }
  ],

  async run(bot, message, args) {
    let cmd;
    if (args[0]) {
      cmd = bot.commands.get(args[0]);
      if (!cmd)
        return message.reply(`Error: \`${args[0]}\` is not a command.`);
    }

    let Embed;
    if (!cmd) {
      let categories = [];
      bot.commands.forEach(command => {
        if (!categories.includes(command.category)) categories.push(command.category);
      })

      Embed = new Discord.EmbedBuilder()
        .setColor(bot.color)
        .setTitle(`Commands`)
        .setAuthor({
          name: message.member.user.username,
          iconURL: message.member.user.displayAvatarURL({ dynmaic: true })
        })
        .setThumbnail(bot.user.displayAvatarURL({ dynamic: true }))
        .setDescription(`Available commands: \`${bot.commands.size}\`\nAvailable categories: \`${categories.length}\``)
        .setTimestamp()
        .setFooter({
          text: "/help",
          iconURL: bot.user.displayAvatarURL({ dynamic: true })
        });

      categories.sort().forEach(async cat => {
        let commands = bot.commands.filter(cmd => cmd.category === cat);
        Embed.addFields({ name: `${cat}`, value: `${commands.map(cmd => `\`${cmd.name}\`: \*\*${cmd.description}\*\*`).join("\n")}`, inline: false });
      });
    } else {
      Embed = new Discord.EmbedBuilder()
        .setColor(bot.color)
        .setTitle(`\`${cmd.name}\` command`)
        .setAuthor({
          name: message.member.user.username,
          iconURL: message.member.user.displayAvatarURL({ dynamic: true })
        })
        .setThumbnail(bot.user.displayAvatarURL({ dynamic: true }))
        .setTimestamp()
        .setFooter({
          text: `/help ${cmd.name}`,
          iconURL: bot.user.displayAvatarURL({ dynamic: true })
        });

      let context = cmd.context;
      if (typeof context !== "string") {
        if (context === 0) context = "Guild"
        else if (context === 1) context = "DM"
        else if (context === 2) context = "PrivateDM"
        else context = "Error: Context"
      }

      Embed.addFields({ name: `Name`, value: `\`${cmd.name}\``, inline: true });
      Embed.addFields({ name: `Description`, value: `\`${cmd.description}\``, inline: false });
      Embed.addFields({
        name: `Requied Permission`,
        value: `\`${typeof cmd.permission !== "bigint" ?
          cmd.permission : new Discord.PermissionsBitField(cmd.permission).toArray()}\``,
        inline: true
      });
      Embed.addFields({ name: `Context`, value: `\`${context}\``, inline: true });
      Embed.addFields({ name: `Category`, value: `\`${cmd.category}\``, inline: true });
    }

    await message.reply({ embeds: [Embed] });
  }
}
