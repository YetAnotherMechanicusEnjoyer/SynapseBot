const fs = require("fs")

module.exports = async (twitchClient) => {

  console.log("[⏳]\x1b[33mTwitch Commands Loading...\x1b[0m");
  fs.readdirSync("./src/cmds").filter(f => f.endsWith(".mjs")).forEach(async file => {

    let command = require(`../cmds/${file}`).command;
    if (!command.name || typeof command.name !== "string") throw new TypeError(`Command ${file.slice(0, file.length - 4)} doesn't have a name !`);
    twitchClient.commands.push(command);
    console.log(`   [✅]\x1b[32m${file.slice(0, file.length - 4)}\x1b[0m`);
  })
}
