mod admin;
mod auto_role;
mod auto_role_group;
mod economy;
mod xp;
mod xp_channel;
mod xp_role;

use crate::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    let mut commands = vec![];
    commands.extend(economy::commands());
    commands.extend(xp::commands());
    commands.extend(admin::commands());
    commands.extend(xp_channel::commands());
    commands.extend(xp_role::commands());
    commands.extend(auto_role::commands());
    commands.extend(auto_role_group::commands());
    commands
}
