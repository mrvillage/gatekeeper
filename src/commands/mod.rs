mod admin;
mod economy;
mod xp;
mod xp_channel;

use crate::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    let mut commands = vec![];
    commands.extend(economy::commands());
    commands.extend(xp::commands());
    commands.extend(admin::commands());
    commands.extend(xp_channel::commands());
    commands
}
