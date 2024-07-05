use poise::serenity_prelude::ChannelId;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use tracing::{debug, info};

static XP_TO_LEVEL: once_cell::sync::Lazy<Vec<i32>> =
    once_cell::sync::Lazy::new(|| (0..=100).map(_xp_to_level).collect());
static XP_AT_LEVEL: once_cell::sync::Lazy<Vec<i32>> =
    once_cell::sync::Lazy::new(|| (0..=100).map(_xp_at_level).collect());

fn _xp_to_level(mut level: i32) -> i32 {
    if level == 0 {
        return 0;
    }
    // this actually calculates the xp required to pass the level, level - 1 makes
    // it so that it's the xp required to reach the level instead
    level -= 1;
    5 * level.pow(2) + 50 * level + 100
}

pub fn xp_to_level(level: i32) -> i32 {
    if level <= 0 {
        return 0;
    }
    if level > 100 {
        return _xp_to_level(100);
    }
    XP_TO_LEVEL[level as usize]
}

fn _xp_at_level(level: i32) -> i32 {
    (0..=level).map(xp_to_level).sum()
}

pub fn xp_at_level(level: i32) -> i32 {
    if level <= 0 {
        return 0;
    }
    if level > 100 {
        return _xp_at_level(100);
    }
    XP_AT_LEVEL[level as usize]
}

pub fn level_up(member: &mut entity::member::ActiveModel) -> bool {
    let xp = member.xp.clone().unwrap();
    let level = member.level.clone().unwrap();
    let xp_to_next_level = xp_at_level(level + 1);
    if xp >= xp_to_next_level {
        debug!(?member, ?level, "leveling up");
        member.level = Set(level + 1);
        level_up(member);
        return true;
    }
    false
}

pub fn level_down(member: &mut entity::member::ActiveModel) -> bool {
    let xp = member.xp.clone().unwrap();
    let level = member.level.clone().unwrap();
    let xp_to_this_level = xp_at_level(level);
    if xp < xp_to_this_level {
        member.level = Set(level - 1);
        level_down(member);
        return true;
    }
    false
}

const INVALID_STARTS: [char; 28] = [
    '!', '?', '.', ',', ';', ':', ' ', '(', ')', '{', '}', '[', ']', '<', '>', '\\', '/', '|', '@',
    '#', '$', '%', '^', '&', '*', '+', '=', '~',
];

pub fn xp_from_message(message: &str) -> i32 {
    if message.is_empty() || message.starts_with(INVALID_STARTS) || message.starts_with("http") {
        return 0;
    }
    ((message.len() / 7) as i32).max(1)
}

pub async fn can_earn_xp(
    db: &DatabaseConnection,
    id: ChannelId,
) -> Result<bool, sea_orm::error::DbErr> {
    let channel = entity::xp_channel::Entity::find_by_id(id.to_string())
        .one(db)
        .await?;
    Ok(channel.is_some())
}
