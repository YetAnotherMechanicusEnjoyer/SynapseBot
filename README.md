# Synapse Bot

<p align="center">
  <img src="./readme/example1.png" alt="Example 1"> <img src="./readme/example2.png" alt="Example 2">
</p>

## Table of Content

- [About](#about)
- [Installation](#installation)
  - [Dependencies](#dependencies)
  - [Discord Bot](#discord-bot)
  - [Twitch App](#twitch-app)
  - [Initialization](#initialization)
- [Usage](#usage)
  - [Twitch](#twitch)
  - [Bot](#bot)
- [Licence](#licence)

## About

> [!NOTE]
> A Twitch App & Discord Bot that links Twich Chat with a Discord Channel

> [!CAUTION]
> I'm currently changing to Rust, go to the [JS](https://github.com/YetAnotherMechanicusEnjoyer/SynapseBot/tree/js) branch if you want to use it correctly.

## Installation

### Dependencies

> [!IMPORTANT]
> Make sure to have [Rust](https://www.rust-lang.org/tools/install) installed.

### Discord Bot

> [!IMPORTANT]
> Make a Discord Bot on the [Discord Developper Portal](https://discord.com/developers/active-developer).
>
> Then, invite it to your Discord server.

### Twitch App

> [!IMPORTANT]
> Make a Twitch App on the [Twitch Console](https://dev.twitch.tv/console).
>
> Then, make an oauth token in [Twitch Token Generator](https://twitchtokengenerator.com) (you must be connected with your App's account).

### Initialization

> [!NOTE]
> Clone the repo somewhere and make a `.env` file at the root of the repository that contains the followig variables.

> [!TIP]
> Exemple of a `.env` file :

```env
TWITCH_BOT_USERNAME="your-twitch-bot-username"
TWITCH_OAUTH_TOKEN="oauth:your-twitch-oauth-token"
TWITCH_CHANNEL="streamers-channel-name"
DISCORD_BOT_TOKEN="your-discord-bot-token"
DISCORD_CHANNEL_ID="your-discord-channel-id"
```

> [!NOTE]
> Compile the binary with [Cargo](https://doc.rust-lang.org/cargo/).

```bash
cargo build --release
```

## Usage

### Twitch

> [!TIP]
> Mod the app on your Twitch Chat to make Twitch API prioritize the App (optionnal).

```
/mod your-twitch-bot-username
```

### Bot

> [!NOTE]
> Run both the Twitch App & Discord Bot with [Cargo](https://doc.rust-lang.org/cargo/) after building it, or execute the binary.

Cargo :
```bash
cargo run --release
```

Binary :
```bash
chmod +x /path/to/repo/target/release/SynapseBot
/path/to/repo/target/release/SynapseBot
```

## Licence

[MIT](https://github.com/YetAnotherMechanicusEnjoyer/SynapseBot/blob/471d506d441951272afa4067d1dc75349af5f129/LICENSE)
