module.exports = async (discordClient, twitchClient) => {
  twitchClient.on('message', (_channel, tags, message, self) => {
    if (self) return;
    const discordChannel = discordClient.channels.cache.get(process.env.DISCORD_CHANNEL_ID);

    if (discordChannel) {
      discordChannel.send(`\`${tags['display-name']}\` **(Twitch):** ${message}`);
    } else {
      console.error(`Error: Channel ${discordChannel} not found.`);
    }
  });

  twitchClient.on('subscription', (_channel, username, _method, _message, tags) => {
    const months = tags['msg-param-cumulative-months'];
    const streak = tags['msg-param-streak-months'];

    const discordChannel = discordClient.channels.cache.get(process.env.DISCORD_CHANNEL_ID);

    if (discordChannel) {
      let subMessage = `### 🎉 A new subscriber! __\`${username}\`__ just subscribed to the channel!`;
      if (months) {
        subMessage = `### ✨ Re-sub from __\`${username}\`__! This is their ${months} month in a row!`;
        if (streak) {
          subMessage += ` (Streak: ${streak} months)`;
        }
      }

      discordChannel.send(subMessage)
        .then(() => {
          console.log(`Relayed subscription notification to Discord: "${subMessage}"`);
        })
        .catch(error => {
          console.error('Error relaying subscription to Discord:', error);
        })
    } else {
      console.error(`Error: Channel ${discordChannel} not found.`);
    }
  })

  twitchClient.on('resub', (_channel, username, months, _message, userstate) => {
    const streak = userstate['msg-param-streak-months'];

    const discordChannel = discordClient.channels.cache.get(process.env.DISCORD_CHANNEL_ID);

    if (discordChannel) {
      let subMessage = `### 🎉 A new subscriber! __\`${username}\`__ just subscribed to the channel!`;
      if (months) {
        subMessage = `### ✨ Re-sub from __\`${username}\`__! This is their ${months} month in a row!`;
        if (streak) {
          subMessage += ` (Streak: ${streak} months)`;
        }
      }

      discordChannel.send(subMessage)
        .then(() => {
          console.log(`Relayed subscription notification to Discord: "${subMessage}"`);
        })
        .catch(error => {
          console.error('Error relaying subscription to Discord:', error);
        })
    } else {
      console.error(`Error: Channel ${discordChannel} not found.`);
    }
  })

  twitchClient.on('subgift', (_channel, username, _streakMonths, recipient, _tags) => {
    const discordChannel = discordClient.channels.cache.get(process.env.DISCORD_CHANNEL_ID);

    if (discordChannel) {
      const giftMessage = `### 🎁 __\`${username}\`__ just gifted a sub to __\`${recipient}\`__!`;

      discordChannel.send(giftMessage)
        .catch(error => console.error('Error relaying gift sub:', error));
    } else {
      console.error(`Error: Channel ${discordChannel} not found.`);
    }
  });
}
