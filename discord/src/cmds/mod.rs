mod duel;
mod help;
mod ping;

pub fn get_all_commands() -> Vec<poise::Command<super::Data, super::Error>> {
    vec![ping::ping(), help::help()]
}
