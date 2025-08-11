const fs = require("fs")

module.exports = async discordClient => {

  console.log("[⏳]\x1b[33mCommands Loading...\x1b[0m");
  fs.readdirSync("./src/appcmds/").filter(f => f.endsWith(".mjs")).forEach(async file => {

    let command = require(`../appcmds/${file}`).command;
    if (!command.name || typeof command.name !== "string") throw new TypeError(`Command ${file.slice(0, file.length - 4)} doesn't have a name !`)
    discordClient.commands.set(command.name, command)
    console.log(`   [✅]\x1b[32m${file.slice(0, file.length - 4)}\x1b[0m`);
  })
}
